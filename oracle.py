'''
# The code to call the fuzzer
'''
import logging
import os
import subprocess
import tempfile
import json
import settings
from collections import defaultdict
from typing import Any, Optional, Tuple, List, Dict


def get_path(path: str) -> str:
    '''
    Check if the path exists and return it
    '''
    if not os.path.exists(path):
        raise RuntimeError(f"{path} not found")
    return path


if not os.path.exists("Differential_Tester"):
    raise RuntimeError("Expect verifier")

instrumentors = {
    "rust": "Differential_Tester/.bin/instrument-rust_2.0/release/instrument",
    "c": "Differential_Tester/.bin/instrument-c_2.0/release/instrument",
}

for language, path in instrumentors.items():
    if not os.path.exists(path):
        raise RuntimeError(f"Missing instrumentor for {language}")

n_counter_examples = 1000


# requires installing the following: sudo yum install -y gcc10.x86_64 gcc10-c++.x86_64
def instrument_c(src_file: str, tmp_dir: str) -> None:
    '''
    Instrumenting C language source files and building a shared library to generate dynamic links
    Args:
        Src_file: C code (json) source code path
        tmp_dir:   Temporary file path
    Returns:

    '''
    # Call the instrumentantors ["c"] tool to process src_file (generate a C test project) and output the result to the tmp_ir/ground_truth directory
    subprocess.check_call([instrumentors["c"], "-f", src_file, "-o", tmp_dir + "/ground_truth"])  # Generate ground_truth file

    subprocess.check_call(  # Compile the project using CMAKE and generate a build file
        [
            "cmake","-DCMAKE_CXX_COMPILER=/usr/bin/g++",
            "-S",tmp_dir + "/ground_truth", # Specify source directory
            "-B",tmp_dir + "/ground_truth/_build", # Specify the build directory
            "-Wno-dev"  # Add this option to ignore warnings
        ]
    )
    # Build the project, execute the actual construction process, and generate corresponding binary files or library files based on the configuration
    subprocess.check_call(["cmake", "--build", tmp_dir + "/ground_truth/_build"])  # Generate libground_truth.so file
    #   tmp_dir + "/ground_truth/_build/libground_truth.so" Move the file to the tmp_ir directory
    subprocess.check_call(["mv", tmp_dir + "/ground_truth/_build/libground_truth.so", tmp_dir])


def instrument(
    language: str, res_dir: str, submodule_name: str, output_dir: str
) -> None:
    """
        Automatically generate Bolero test Rust code
    Args:
        language (str): The source code language
        res_dir (str): The path to C/Go code, JSON files, and Rust code (temporary directory) -> [res_dir/submodule_name.{c/go,json,rs}]
        submodule_name (str): The submodule name
        output_dir (str): The output file path. (workspace = res_dir/replay)

        Returns:
            None.
        Raises:
            FileExistsError: If output_dir already exists.
            CalledProcessError: If instrumentation fails.
        """
    logging.info(f"Instrumenting {submodule_name}")
    rs_file: str = get_path(f"{res_dir}/{submodule_name}.rs")
    if os.path.exists(output_dir):
        raise FileExistsError(
            f"output directory {output_dir} exists, cannot instrument {submodule_name}"
        )
    with tempfile.TemporaryDirectory() as tmp_dir:
        src_file: str
        if language == "c":
            src_file = get_path(f"{res_dir}/{submodule_name}.json")
            instrument_c(src_file, tmp_dir)
        else:
            raise NotImplementedError

        # Process a Rust source code file using the instrumentantors ["rust"] variable and generate a rust test project in the output directory
        subprocess.check_call(
            [
                instrumentors["rust"],
                "-f",rs_file,
                "-o",output_dir,
                "--capture-stdout",
                "--wrapper-structs",
                "--arbitrary-precision",
                "--ground-truth",tmp_dir + "/libground_truth.so",
                "--multi-examples",str(n_counter_examples),
            ]
        )
        # Move files
        subprocess.check_call(["mv", tmp_dir + "/libground_truth.so", output_dir])


def verify(
    fuzz_target: str, submodule_name: str, result_path: Optional[str] = None
) -> Optional[Tuple[str, str]]:
    """
    Perform fuzz testing and return positive and negative examples
    Args:
        fuzz_target (str): The path to the fuzz target. (workspace)
        submodule_name (str): The name of the submodule
        result_path (Optional[str]): An optional result path. If provided, crash_report, positive_examples, and counter_examples will be written to the file
    Returns:
        None: If oracle generation fails.
        Tuple[positive_examples, counter_examples]: A pair of positive/negative examples. (E+, E-)
    """
    fuzz_target: str = get_path(os.path.abspath(fuzz_target))
    logging.info(f"Start verifying {submodule_name}")

    env = os.environ.copy()
    env["LD_LIBRARY_PATH"] = fuzz_target
    env["RUSTFLAGS"] = f"-L {fuzz_target}"

    # Retrieve test related information from the specified Rust project and print the name (or identifier) of the first test to the terminal
    main_entry = (
        subprocess.run(
            "cargo bolero list --manifest-path " # Extract specific information from the Cargo.toml file of the Cargo project
            f"{fuzz_target}/Cargo.toml | jq '.test' | head -n 1 | xargs echo ", # Pass the extracted information to the jq tool and use. test to filter out the. test field
            shell=True,
            capture_output=True,
            env=env,
        )
        .stdout.decode("utf-8")
        .strip()
    )

    if len(main_entry) == 0:
        logging.info("len(main_entry)...")
        return None

    VERIFICATION_TIMEOUT = 720  # default = 420
    RETRY_LIMIT = 1
    # exponential backoff...
    init_max_len = 32768  # default = 32768
    retry_cnt = 0
    timeout = VERIFICATION_TIMEOUT
    while True:
        # Perform fuzz testing on the Cargo project
        verification = subprocess.Popen(
            f"cargo bolero test --manifest-path {fuzz_target}/Cargo.toml " # Execute testing tasks
            f"--features fuzzing {main_entry} --target-dir {fuzz_target}/target/__fuzz__ " # Using the fuzzing feature to build a project, {main_detry} may be an entry point or other configuration parameter
            "--sanitizer NONE "  # Disable any memory checker
            f'--engine-args="-rss_limit_mb=11264 -max_len={init_max_len}" ',  # Set the RSS memory limit to 8096 MB and the maximum length parameter, default = 8096
            shell=True,
            stdout=subprocess.DEVNULL,  # Disable the standard output of child processes
            stderr=subprocess.PIPE,
            env=env,
        )
        try:
            #Ensure that the errs variable obtained after the child process completes contains the complete standard error output without losing any important information
            _, errs = verification.communicate(timeout=timeout)  # Read all standard outputs and standard errors at once after the child process is completed
            crash_report = errs.decode("utf-8").strip()
            break
        except subprocess.TimeoutExpired: # Capture timeout exception of communication method
            verification.kill() # Force termination of child process
            if retry_cnt == RETRY_LIMIT: # Check retry counter
                return None
            logging.info("Verification timeout. Increasing max input size.")
            retry_cnt += 1
            init_max_len *= 4 # Multiply the initial maximum input size by 4 to increase the input size
            timeout *= 2
            continue

    positive_examples: Optional[str] = None
    counter_examples: Optional[str] = None
    for line in crash_report.splitlines():
        if line.startswith("positive examples: "):
            positive_examples = line[len("positive examples: ") :]
        elif line.startswith("counter examples: "):
            counter_examples = line[len("counter examples: ") :]

    if not positive_examples or not counter_examples:
        return None

    if result_path:
        os.makedirs(f"{result_path}/verify_result", exist_ok=True)
        with open(result_path + "/verify_result/crash_report.log", "w") as f:
            f.write(crash_report)
        with open(result_path + "/verify_result/counter_examples.txt", "w") as f:
            f.write(counter_examples)
        with open(result_path + "/verify_result/positive_examples.txt", "w") as f:
            f.write(positive_examples)

    return positive_examples, counter_examples


rust_sysroot = (
    subprocess.run(
        "rustc --print sysroot",
        capture_output=True,
        shell=True,
        check=True)
    .stdout.decode("utf-8")
    .strip()
)

llvm_cov = (
    subprocess.run(
        f'find {rust_sysroot} -name "llvm-cov" | head -n 1',
        capture_output=True,
        shell=True,
        check=True,
    )
    .stdout.decode("utf-8")
    .strip()
)
llvm_profdata = (
    subprocess.run(
        f'find {rust_sysroot} -name "llvm-profdata" | head -n 1',
        capture_output=True,
        shell=True,
        check=True,
    )
    .stdout.decode("utf-8")
    .strip()
)


def parse_llvm_cov_show(target_dir: str, show: str) -> List[Tuple[str, str]]:
    """
    Parse the output string from the llvm-cov show command and extract execution count and program section information
    Args:
        target_dir (str): The path to the target directory, used to locate relevant sections.
        show (str): The output string processed by the llvm-cov show command

    Returns:
        List[Tuple[str, str]]: A list containing execution counts and corresponding program sections,
        with each element being a tuple (exec_count, program_part) -> (execution count, program section information)
    """

    # This is a hack.
    show = show.split(f"{target_dir}/src/lib.rs:\n")[1]
    parts = []
    start = False
    for line in show.splitlines():
        if not line.strip(): # Check if the variable line is empty or contains only whitespace characters
            continue
        if line.strip()[0].isdigit(): # Is the first character after removing the leading and trailing blank characters a number
            if "mod communication {" in line:
                break
            if 'extern "C" {' in line:
                start = True
            if not start:
                continue
            _, exec_count, program_part = line.split("|", 2)
            parts.append((exec_count, program_part))

    return parts


def test_cases_cov_info(replay_dir: str, io_examples: str) -> List[int]:
    """
    Analyze the coverage information of a set of I/O examples and return the information on how many examples cover each line of code
    Args:
        replay_target (str): The path to the replay directory, used to specify the location of the replay target.
        io_examples (str): A string containing I/O examples in JSON list format.

    Returns:
        cov_data: The number of examples that cover each line of code.
        processed_lines: A list of processed lines, as the line numbers in the original code may not match those in the coverage report.
     """
    io_examples = json.loads(io_examples)
    cov_mat = []
    processed_lines: List[str] = []
    for io_example in io_examples:
        _, show = compute_coverage(replay_dir, json.dumps([io_example]))

        def parse_exec_count(s: str) -> int:
            try:
                return int(s)
            except ValueError:
                return 0

        cov: List[int] = list(
            map(lambda line_info: parse_exec_count(line_info[0]), show)
        )
        cov_mat.append(cov)

        processed_lines = list(
            map(lambda line_info: line_info[1], show)
        )  # same for every loop iteration

    # transpose cov matrix: line -> io_example -> int
    cov_mat = [list(l) for l in zip(*cov_mat)]

    cov_data = [sum(1 for cnt in line_info if cnt > 0) for line_info in cov_mat]

    return cov_data, processed_lines


def group_examples_by_coverage(
    replay_dir: str, negative_examples: str, N_EXAMPLES: int, early_stop: bool = True
) -> Dict[List[int], Any]:
    '''
    Group negative examples based on coverage information.
    Args:
        replay_dir: The path to the replay directory, used to specify the location of the replay target.
        negative_examples: A JSON formatted string containing a list of negative examples.
        N_EXAMPLES: The number of examples required in each coverage group.
        early_stop: A boolean value, optional parameter, default is True, indicating whether to stop early once enough examples are found.

    Returns:
        cov_to_ce: A dictionary of negative examples grouped by coverage information.
    '''
    negative_examples = json.loads(negative_examples)
    cov_to_ce = defaultdict(list)
    for example in negative_examples:  # example(dict[3])
        l_cov = []
        # Using Rust compiler and LLVM toolchain, combined with input test examples, calculate and analyze the code coverage of the target program
        _, ex_data = compute_coverage(replay_dir, str([example]))
        for ex_d in ex_data:
            try:
                ex_cnt = int(ex_d[0])
                if ex_cnt > 0:
                    l_cov.append(1)
                else:
                    l_cov.append(0)
            except:
                l_cov.append(0)
        cov_to_ce[str(l_cov)].append(example)
        if early_stop and len(cov_to_ce[str(l_cov)]) == N_EXAMPLES:
            return {str(l_cov): cov_to_ce[str(l_cov)]}

    return cov_to_ce

def compute_coverage_by_libfuzzer_corpus(
    fuzz_target: str,
) -> Tuple[str, List[Tuple[str, str]]]:
    """
    Calculate coverage using libFuzzer's corpus
    Args:
        fuzz_target (str): The path to the fuzzing target
    Returns:
        Tuple[str, List[Tuple[str, str]]]: A pair of report/show.
    """
    instrument_flags = (
        # "-Zunstable-options -C instrument-coverage=except-unused-functions"
        "-Zunstable-options -C instrument-coverage=on"
    )
    env = os.environ.copy()
    env["RUSTFLAGS"] = instrument_flags

    test_bin = (
        subprocess.run(
            f"cargo test --manifest-path {fuzz_target}/Cargo.toml "
            '--tests --no-run --message-format=json | jq -r "select(.profile.test == true) '
            '| .filenames[]" | grep -v dSYM -',
            capture_output=True,
            shell=True,
            check=True,
            env=env,
        )
        .stdout.decode("utf-8")
        .strip()
    )

    subprocess.run(
        f"cargo test --manifest-path {fuzz_target}/Cargo.toml",
        shell=True,
        capture_output=True,
        env=env,
    )

    subprocess.call(
        f"{llvm_profdata} merge -sparse {fuzz_target}/*.profraw -o {fuzz_target}/cov.profdata",
        shell=True,
    )

    report = (
        subprocess.run(
            f"{llvm_cov} report -instr-profile={fuzz_target}/cov.profdata {test_bin}",
            shell=True,
            capture_output=True,
        )
        .stdout.decode("utf-8")
        .strip()
    )

    show = (
        subprocess.run(
            f"{llvm_cov} show -instr-profile={fuzz_target}/cov.profdata {test_bin} "
            "--show-instantiations --show-line-counts-or-regions",
            shell=True,
            capture_output=True,
        )
        .stdout.decode("utf-8")
        .strip()
    )

    return report, parse_llvm_cov_show(fuzz_target, show)


def compute_coverage(
    replay_dir: str, io_examples: str
) -> Tuple[str, List[Tuple[str, str]]]:
    """
Calculate code coverage based on the given I/O examples and return the relevant report and parsed coverage information
    Using Rust's compiler and LLVM toolchain, combined with the input test examples, calculate and analyze the code coverage of the target program
    Args:
        replay_dir (str): The path to the directory containing the test target and related files.
        io_examples (str): A string representation of a list of examples used for testing. str([example])
    Returns:
        report: A string containing the generated code coverage report
        List[Tuple[str, str]]: A list containing execution counts and corresponding program sections, with each element being a tuple (exec_count, program_part) -> (execution count, program section information)
    """
    instrument_flags = (
        "-Zunstable-options -C instrument-coverage=on"
    )
    env = os.environ.copy()
    env["RUSTFLAGS"] = instrument_flags

    # remove possible previous data
    subprocess.run(f"rm -f {replay_dir}/*.profraw", shell=True)
    subprocess.run(f"rm -f {replay_dir}/cov.profdata", shell=True)
    # Run Rust test commands to obtain the paths of all test binary files and return their paths as strings.
    test_bin = (
        subprocess.run(
            f"cargo test --manifest-path {replay_dir}/Cargo.toml --features replay "
            '--tests --no-run --message-format=json | jq -r "select(.profile.test == true) '
            '| .filenames[]" | grep -v dSYM -',
            capture_output=True,
            shell=True,
            check=True,
            env=env,
        )
        .stdout.decode("utf-8")
        .strip()
    )
    # When executing the cargo test command, pass specific input data to the test program without displaying any output information
    subprocess.run(
        f"cargo test --manifest-path {replay_dir}/Cargo.toml --features replay",
        stderr=subprocess.DEVNULL,
        stdout=subprocess.DEVNULL,
        input=io_examples.encode(),
        shell=True,
        env=env,
    )
    # Merge multiple. fraw files generated by LLVM coverage tool into a sparse. pdf file
    subprocess.call(
        f"{llvm_profdata} merge -sparse {replay_dir}/*.profraw -o {replay_dir}/cov.profdata",
        shell=True,
    )
    # Call LLVM's LLVM cov tool, use the previously generated code coverage data file, analyze the specified test binary file, and obtain its generated code coverage report
    report = (
        subprocess.run(
            f"{llvm_cov} report -instr-profile={replay_dir}/cov.profdata {test_bin}",
            shell=True,
            capture_output=True,
        )
        .stdout.decode("utf-8")
        .strip()
    )
    # Call LLVM's LLVM cov tool and utilize the previously generated code coverage data file,
    # Display detailed code coverage information for the specified test binary file, including template instantiation information, line coverage, or area coverage, etc
    show = (
        subprocess.run(
            f"{llvm_cov} show -instr-profile={replay_dir}/cov.profdata {test_bin} "
            "--show-instantiations --show-line-counts-or-regions",
            shell=True,
            capture_output=True,
        )
        .stdout.decode("utf-8")
        .strip()
    )

    return report, parse_llvm_cov_show(replay_dir, show)


def soft_verify(
    replay_target: str,
    submodule_name: str,
    positive_examples: str,
    counter_examples: str,
) -> Optional[Tuple[str, str]]:
    """
        Re-validate the target with a given set of I/O examples. [For positive and negative examples]
    Args:
        replay_target (str): The path to the replay target.
        submodule_name (str): The name of the submodule.
        positive_examples (str): A set of positive examples.
        counter_examples (str): A set of counterexamples.
    Returns:
        None: If oracle generation fails.
        Tuple[str, str]: A pair of positive/negative examples. (E+, E-)
    """
    replay_target: str = get_path(os.path.abspath(replay_target))
    logging.info(f"Start soft-verifying {submodule_name}")

    pe_typck: list = json.loads(positive_examples)
    ce_typck: list = json.loads(counter_examples)

    io_examples = json.dumps(ce_typck + pe_typck)

    VERIFICATION_TIMEOUT = 300
    timeout = VERIFICATION_TIMEOUT
    crash_report: str
    verification = subprocess.Popen(
        f"cargo test --manifest-path {replay_target}/Cargo.toml --features replay -- --nocapture",
        shell=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        stdin=subprocess.PIPE,
    )
    try:
        _, errs = verification.communicate(input=io_examples.encode(), timeout=timeout)
        crash_report = errs.decode("utf-8").strip()
    except subprocess.TimeoutExpired:
        verification.kill()
        return None

    new_positive_examples: Optional[str] = None
    new_counter_examples: Optional[str] = None
    for line in crash_report.splitlines():
        if line.startswith("positive examples: "):
            new_positive_examples = line[len("positive examples: ") :]
        elif line.startswith("counter examples: "):
            new_counter_examples = line[len("counter examples: ") :]

    if not new_positive_examples or not new_counter_examples:
        return None

    return new_positive_examples, new_counter_examples
