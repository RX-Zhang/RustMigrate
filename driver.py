import os
import logging
import numpy as np

from argparse_dataclass import ArgumentParser
from typing import List, Tuple, Optional
import shutil

from fixer import Fixer
from llms import QueryEngineFactory
from transpiler import Transpiler
from settings import Options

from semantics import Candidate, CandidateFactory, SemanticsStrategy


def record_cov_data(report: str, show: List[Tuple[str, str]], work_dir: str):
    '''
        Write the coverage report and execution information to a text file in the specified directory
    Args:
        report: The string content of the coverage report
        show: A list containing execution count and line information, where each element is a tuple (exec_count, line)
        work_dir: The path of the working directory, used to determine the file storage location
    '''
    os.makedirs(f"{work_dir}/cov", exist_ok=True)
    with open(f"{work_dir}/cov/cov_report.txt", "w") as f:  # work_dir/cov_report.txt
        f.write(report)

    with open(f"{work_dir}/cov/cov_show.txt", "w") as f:  # work_dir/cov_show.txt
        for exec_count, line in show:
            print(exec_count + "|" + line, file=f)


def construct_factory(options: Options) -> CandidateFactory:
    '''
        Construct a CandidateFactory object based on the given options parameter and return the object
    Args:
        options: Contains the configuration information required to build the CandidateFactory object

    Returns:
        CandidateFactory
    '''
    src_code: str
    src_code_json: str
    with open(f"{options.res_dir}/{options.submodule_name}.{options.language}","r") as f:
        src_code = f.read()
    with open(f"{options.res_dir}/{options.submodule_name}.json","r") as f:
        src_code_json = f.read()

    factory = CandidateFactory(
        src_code,
        src_code_json,
        options.language,
        options.submodule_name,
        options.sem_fix,
        options.work_dir
    )
    return factory


def latest_rust_code(options: Options) -> str:
    '''
        Read the latest Rust code
    Args:
        options: Contains the configuration information required by the function
    Returns:
        rust_code: The content of the file returned as a string.
    '''
    rust_code: str
    with open(f"{options.res_dir}/{options.submodule_name}.rs", "r") as f:
        rust_code = f.read()
    return rust_code


def initial_transpilation(
    transpiler: Transpiler, options: Options
) -> Optional[Tuple[Candidate, CandidateFactory]]:
    '''
        Attempt the initial transpilation operation and return a valid candidate object
    Args:
        transpiler: The transpiler object used to perform the code transpilation.
        options: The options object containing configuration and parameter information.

    Returns:
        If a transpilable result is found:
            candidate: The valid transpilation candidate object.
            factory: The factory object used to build the candidate.
        If no valid transpilation result is found (after reaching the maximum retry attempts):
            Returns None.
    '''
    INIT_ATTEMPT_BUDGTE = 3 # Number of translations
    for _ in range(INIT_ATTEMPT_BUDGTE):
        compiles = transpiler.transpile()
        if compiles:
            # Check semantic equivalence.
            logging.info("Found a compiling transpilation. Checking semantic equivalence...")

            factory = construct_factory(options)  # Create a factory and retrieve the source code and corresponding JSON file from the file
            rust_code = latest_rust_code(options)  # Obtain the translated Rust source code
            candidate = factory.construct_candidate(rust_code)

            if not candidate:
                continue

            return candidate, factory
        else:
            logging.info("Candidate does not compile. Retrying.")
    return None


def main():
    parser = ArgumentParser(Options)
    options = parser.parse_args()

    global_constraints = []
    if options.language == "c":
        global_constraints.append("Consider using wrapping operations like wrapping_add, wrapping_shr, etc., to simulate C semantics.")

    # Create a translation engine
    query_engine = QueryEngineFactory.create_engine(options.model, global_constraints)

    # clear work_dir
    if os.path.exists(options.work_dir):
        shutil.rmtree(options.work_dir)
    os.makedirs(options.work_dir)

    # Create log related configurations
    logging.basicConfig(
        filename="%s/transpilation.log" % options.work_dir,
        level=logging.INFO,
        filemode="w",
        format="%(name)s - %(levelname)s - %(message)s",
    )
    logging.info("%s transpilation has started." % options.benchmark_name)

    rng = np.random.default_rng(123)

    # Instantiate repair class
    if options.comp_fix == "no":
        comp_fixer = None
    else:
        comp_fixer = Fixer(options.comp_fix, query_engine, global_constraints, options.comp_fix_attempt_budget)
    eq_fixer = None

    fallback = options.fallback_opt  # "fix"
    restart_budget = options.restart_budget
    fix_budget = options.fix_budget
    # Instantiate translation class
    transpiler = Transpiler(
        "base",
        comp_fixer,
        eq_fixer,
        options.language,
        options.benchmark_name,
        options.submodule_name,
        query_engine,
        options.transpl_attempt_budget,
        options.work_dir,
        global_constraints,
        model_params={"temperature": options.initial_temperature},
    )

    # First time generating Rust translation through LLM, conducting fuzz testing, and then building a candidate object to return
    transpilation = initial_transpilation(transpiler, options)

    res_path = f"{options.language}/{options.benchmark_name}/{options.submodule_name}/{options.model}/{options.feedback_strategy}"

    if not transpilation:
        logging.info("Failed to find compilable/checkable candidate. Return Code: 0.")
        if os.path.isdir(options.res_dir):
            rust_code = latest_rust_code(options)
        else:
            with open(f"{options.work_dir}/wspace/src/lib.rs", "r") as f:
                rust_code = f.read()
        if rust_code != "":
            # save results
            res_fail_path = f'translation_result/real_world_code/fail/{options.language}/{options.benchmark_name}/{options.submodule_name}/{options.model}/{options.feedback_strategy}'
            os.makedirs(res_fail_path, exist_ok=True)
            with open(f"{res_fail_path}/{options.submodule_name}.rs", 'w') as file:
                file.write(rust_code)
            with open(f"result_all/{options.benchmark_name}/fail.txt", 'a') as file:
                file.write(res_path)
            print("compile success,but translation failed")
            return
        else:
            print("compile failed")
            return

    candidate, factory = transpilation
    if candidate.ok:
        record_cov_data(*candidate.extra, options.work_dir)
        logging.info("Transpilation finished. Equivalent transpilation has been found at initial attempt. Return Code: 1.")
        # save results
        res_success_path = f'translation_result/real_world_code/success/{options.language}/{options.benchmark_name}/{options.submodule_name}/{options.model}/{options.feedback_strategy}'
        os.makedirs(res_success_path, exist_ok=True)
        with open(f"{res_success_path}/{options.submodule_name}.rs", 'w') as file:
            file.write(candidate.rust_code)
        with open(f"result_all/{options.benchmark_name}/success.txt", 'a') as file:
            file.write(res_path)
        return

    # FALLBACK
    num_oracle_oos = 0
    fixed_once = False
    logging.info(f"Transpilation is not equivalent: candidate score = {candidate.score}.")
    for restart_idx in range(restart_budget):
        if options.hinted and candidate:
            transpiler.hint = candidate.hint(options.n_prompt_examples)  # Add additional suggestive information to LLM
        if fallback == "fix":  # Using LLM to fix semantic inconsistencies in code
            logging.info("Now attempting LLM-based semantics fixing.")
            semantics_strategy = SemanticsStrategy(
                restart_idx,
                factory,
                options,
                query_engine,
                global_constraints,
                budget=fix_budget,
            )
            # Multiple calls to fix using LLM to regenerate semantically consistent Rust code, build a candidate, and then return the best one
            candidate = semantics_strategy.optimize(candidate)
            if candidate.ok:
                fixed_once = True
                logging.info("Current errors has been cleaned. Verifying again.")
                candidate = factory.construct_candidate(candidate.rust_code)
                assert candidate
                if candidate.ok:
                    record_cov_data(*candidate.extra, options.work_dir)
                    logging.info(f"Equivalent transpilation has been found by {fallback} strategy. Return Code: 2")
                    # save results
                    res_success_path = f'translation_result/real_world_code/success/{options.language}/{options.benchmark_name}/{options.submodule_name}/{options.model}/{options.feedback_strategy}'
                    os.makedirs(res_success_path, exist_ok=True)
                    with open(f"{res_success_path}/{options.submodule_name}.rs", 'w') as file:
                        file.write(candidate.rust_code)
                    with open(f"result_all/{options.benchmark_name}/success.txt", 'a') as file:
                        file.write(res_path)
                    return
                else:
                    pass
        else:
            if fallback == "restart":
                pass
            elif fallback == "param-search":
                # Generate a new temperature value based on the current temperature cur_temp and a random offset dev,
                # and ensure that this new temperature value is within the range of [0,1]
                def mutate_temperature(cur_temp, rng):
                    mu, sigma = 0, 0.1
                    dev = rng.normal(mu, sigma)
                    new_temp = max(0, min(1, cur_temp + dev))
                    return new_temp

                temperature = transpiler.model_params["temperature"]
                new_temperature = mutate_temperature(temperature, rng)
                transpiler.model_params["temperature"] = new_temperature
                logging.info(f"Temperature is set to: {new_temperature}")
            elif fallback == "simplify":
                transpiler.prompt = "decomp-iter"  # transpile_decomp_iter

            # recompile
            compiles = transpiler.transpile()
            if not compiles:
                continue
            logging.info("Found a compiling transpilation. Checking semantic equivalence...")
            rust_code = latest_rust_code(options)
            candidate = factory.construct_candidate(rust_code)
            if not candidate:
                num_oracle_oos += 1
            elif candidate.ok:
                record_cov_data(*candidate.extra, options.work_dir)
                logging.info(f"Equivalent transpilation has been found by {fallback} strategy. Restart id: {restart_idx}. Return Code: 2")
                # save results
                res_success_path = f'translation_result/real_world_code/success/{options.language}/{options.benchmark_name}/{options.submodule_name}/{options.model}/{options.feedback_strategy}'
                os.makedirs(res_success_path, exist_ok=True)
                with open(f"{res_success_path}/{options.submodule_name}.rs", 'w') as file:
                    file.write(candidate.rust_code)
                with open(f"result_all/{options.benchmark_name}/success.txt", 'a') as file:
                    file.write(res_path)
                return

    if fixed_once:
        logging.info("Fallback process failed cleaning semantic errors. Return Code: 3")  # special failure case
    elif num_oracle_oos > 0:
        logging.info("Fallback process failed cleaning semantic errors. Return Code: 4")  # Oracle Partially OOS
    elif num_oracle_oos > 5:
        logging.info("Fallback process failed cleaning semantic errors. Return Code: 5")  # Oracle OOS
    else:
        logging.info("Fallback process failed cleaning semantic errors. Return Code: 6")
    # save results
    res_fail_path = f'translation_result/real_world_code/fail/{options.language}/{options.benchmark_name}/{options.submodule_name}/{options.model}/{options.feedback_strategy}'
    os.makedirs(res_fail_path, exist_ok=True)
    with open(f"{res_fail_path}/{options.submodule_name}.rs", 'w') as file:
        file.write(candidate.rust_code)
    with open(f"result_all/{options.benchmark_name}/fail.txt", 'a') as file:
        file.write(res_path)


if __name__ == "__main__":
    main()
    print("\nrun successfully")
