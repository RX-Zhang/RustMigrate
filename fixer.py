'''
# Code used to fix compilation errors during initial compilation
'''
import copy
import random
import logging
from llms import Prompt, QueryEngine
from utils import *
import settings


class Fixer:
    def __init__(
        self, fix_type, query_engine: QueryEngine, global_constraints, comp_fix_attempt_budget=3
    ) -> None:
        self.comp_fix_attempt_budget = comp_fix_attempt_budget
        self.fix_type = fix_type  # options.comp_fix
        self.query_engine = query_engine
        self.global_constraints = global_constraints

    fix_compile_constrains = [
        # "Give me the whole fixed code back, dont add explanation, comment or anything else.",
        "Use only safe Rust.",
        "Don't use raw pointers.",
        "Use box pointer whenever possible. Box pointers are preferable to other alternatives.",
        "Try not to use Traits if possible. I would not like to have Traits in resulting Rust code.",
        "Try not to use Generics if possible.",
    ]

    def fix(self, rust_code="", comp_out=None, work_dir=None):
        '''
        Call the comp_fix_msft_work function to fix Rust code compilation errors
        Args:
            rust_code: The Rust code
            comp_out: A tuple containing the compilation output
            work_dir: The working directory
        Returns:
            rust_code: The fixed Rust code
            len(errors): The number of remaining errors
            num_llm_call: The number of LLM calls
        '''
        self.fix_path = []
        return self.comp_fix_msft_work(rust_code, comp_out, work_dir)

    def cargo_fix(self, work_dir):
        '''
            Use the cargo fix command to automatically fix Rust code
        Args:
            work_dir: The working directory # work_dir/wspace
        '''
        with cd(f"{work_dir}"):
            subprocess.run(f"cargo clean", capture_output=True, shell=True)

            # Compile the project using Rust's build tool Cargo
            comp_output_bf_cfix = subprocess.run(
                f'RUSTFLAGS="-Z track-diagnostics -Z time-passes" cargo build --manifest-path Cargo.toml',
                capture_output=True,
                shell=True,
            )# Return execution status information or error messages
            _, _, _, _, init_num_errors = parse_error_timepass(comp_output_bf_cfix.stderr, work_dir.split("/")[-1])

            # Automatically fix compiler warnings and errors in Rust code.
            subprocess.run(f"cargo fix --allow-no-vcs", capture_output=True, shell=True)

            subprocess.run(f"cargo clean", capture_output=True, shell=True)
            # recompile
            comp_output_af_cfix = subprocess.run(
                f'RUSTFLAGS="-Z track-diagnostics -Z time-passes" cargo build --manifest-path Cargo.toml',
                capture_output=True,
                shell=True,
            )
            _, _, _, _, fnl_num_errors = parse_error_timepass(comp_output_af_cfix.stderr, work_dir.split("/")[-1])

            logging.info(f"\tNumber of errors decreased from {init_num_errors} to {fnl_num_errors} with cargo fix.")

    def comp_fix_msft_work(self, rust_code, init_comp_out, work_dir):
        '''
        Call LLM to fix compilation errors in the given Rust code.
        Args:
            rust_code: The Rust code
            init_comp_out: A tuple containing the compilation output
            work_dir: The working directory
        Returns:
            rust_code: The fixed Rust code
            len(errors): The number of remaining compilation errors
            num_llm_call: The number of LLM (Large Language Model) calls used to fix the compilation errors
        '''
        errors = init_comp_out[0]  # Init_comp_out [0]: Contains a list of all compilation errors.
        num_llm_call = 0
        while errors:
            snap = rust_code
            error = random.choice(errors)

            cur_errors = set([error])
            rep_counter = 0
            while True:
                cur_err = cur_errors.pop()
                # Build prompt message (single error message)
                prompt = Prompt(
                    context=(
                        f"You are given a Rust code contained in <code> tags."
                        + tag(rust_code, "code")
                        + "This code does not compile. Here are some error messages contained in <error-message> tags"
                        + tag(cur_err.body, "error-message")
                    ),
                    instruction="Please correct the issue of inability to compile based on these error messages",
                    constraints=self.fix_compile_constrains,
                )

                # Use LLM to fix code
                rust_code = self.query_engine.generate_code(prompt)

                recode_prompt = copy.deepcopy(prompt)
                recode_prompt.constraints = prompt.constraints + self.global_constraints + settings.test_constraints_prompt
                num_llm_call += 1  # increment before log
                # Compile and output errors
                comp_output = compile_and_record_query(rust_code, work_dir, self.query_engine.stringify_prompt(recode_prompt), f"com_fix_msft_{num_llm_call}")
                # Analysis error
                fnl_comp_out = parse_error_timepass(comp_output.stderr, work_dir.split("/")[-1])
                new_errors = fnl_comp_out[0]
                cur_errors = set(new_errors) - set(errors)  # The element that exists in new_ errors but does not exist in errors

                if not cur_errors:
                    errors = new_errors
                    break

                rep_counter += 1
                if rep_counter == 4:
                    rust_code = snap
                    break
            if num_llm_call >= 10:
                break

        return rust_code, len(errors), num_llm_call