'''
# Code for obtaining initial translation from LLM
'''
import json
import logging
import shutil
import copy

from llms import QueryEngine, Prompt
from utils import *
import settings

class Transpiler:
    def __init__(
        self,
        prompt,
        comp_fixer,
        eq_fixer,  # None
        src_lang,
        benchmark,
        submodule_name,
        query_engine: QueryEngine,
        transpl_attempt_budget,
        work_dir,
        global_constraints,
        model_params={"temperature": 0.2},
    ) -> None:
        self.src_lang = src_lang
        self.benchmark = benchmark
        self.fname = submodule_name  # submodule_name
        self.benchmark_path = f"benchmark/{src_lang}/{benchmark}"
        self.prompt = prompt  #  base
        self.comp_fixer = comp_fixer
        self.eq_fixer = eq_fixer  # None
        self.query_engine = query_engine
        self.transpl_attempt_budget = transpl_attempt_budget
        self.hint = ""
        self.model_params = model_params
        self.work_dir = work_dir
        self.global_constraints = global_constraints

    common_translate_constraints = [
        "Make sure it includes all imports, uses safe rust, and compiles.",
        "Don't use raw pointers.",
        "Use box pointer whenever possible. Box pointers are preferable to other alternatives.",
        "Try not to use Traits if possible. I would not like to have Traits in resulting Rust code.",
        "Try not to use custom Generics if possible.",
    ]

    def transpile(self):
        '''
        self.prompt == base
        Returns:
            compiles results
        '''
        if self.prompt == "base":
            return self.transpile_base()
        elif self.prompt == "decomp-iter":
            return self.transpile_decomp_iter()

    def update_prompt(self, cur_code, cur_answer):
        '''
        Update the prompt message.
        Args:
            cur_code: The current code to be translated
            cur_answer: The previously translated code (only includes import statements, and may also include structures and global variables)
        Returns:
            prompt: The prompt message
        '''
        prompt = Prompt(
            context=f"You are given a {self.src_lang.capitalize()} code contained in <code> tags. We need to translate this code to Rust."
            + tag(cur_code, "code"),
            instruction=f"Give me the Rust translation of the above {self.src_lang.capitalize()} code.",
            constraints=self.common_translate_constraints,
            preamble=cur_answer,
        )

        return prompt

    def transpile_decomp_iter(self):
        '''
        The process of converting specified source code from a language to Rust, compiling it, and fixing errors
        Separate the source code into the main body and other parts for translation, and finally merge them
        '''
        compiles = False
        logging.info(f"Now transpiling {self.fname}.")

        src_dir = f"{self.work_dir}/wspace/"
        res_dir = f"{self.work_dir}/results/"
        source_dict = json.load(open(f"{self.benchmark_path}/{self.fname}/{self.fname}.json", "r"))
        imports = source_dict["Includes"]
        structs = source_dict["Structs"]
        declarations = source_dict["Function Declarations"]
        implementations = source_dict["Function Implementations"]
        enums = source_dict["Enums"]
        defines = source_dict["Defines"]
        type_defs = source_dict["TypeDefs"]
        global_vars = source_dict["Globals"]
        all_aux = ("\n".join(enums) + "\n" + "\n".join(type_defs) + "\n" + "\n".join(defines) + "\n" + "\n".join(global_vars))
        func_name = "header"

        all_imp = "\n".join(imports)
        all_st = "\n".join(structs)
        cur_code = "\n" + all_imp + "\n" + all_aux + "\n" + all_st
        cur_answer = ""
        prompt = Prompt(
            context=f"You are given a {self.src_lang.capitalize()} code contained in <code> tags. "
            + "This code contains only import statements and possibly structs and gloabal variables. We need to translate this code piece to Rust.\n"
            + tag(cur_code, "code"),
            instruction=f"Give me the Rust translation of the above {self.src_lang.capitalize()} code.",
            constraints=self.common_translate_constraints,
        )

        recode_frist_prompt = copy.deepcopy(prompt)
        for func_dec, func_impl in zip(declarations, implementations):
            logging.info(f"   Working on {func_name} function.")
            min_num_errs = 2**32
            best_answer_processed = ""
            recode_frist_prompt.constraints = prompt.constraints + self.global_constraints + settings.test_constraints_prompt
            recode_frist_prompt.preamble = prompt.preamble
            for attempt in range(1, self.transpl_attempt_budget + 1):
                answer = self.query_engine.generate_code(prompt, model_params=self.model_params)

                cand_answer_processed = answer
                comp_out = compile_and_record_query(answer, src_dir, self.query_engine.stringify_prompt(recode_frist_prompt))
                parsed_comp_out = parse_error_timepass(comp_out.stderr, self.fname)
                num_errs = parsed_comp_out[-1]

                logging.info(f"\tAttemp {attempt}: {num_errs} errors.")
                if num_errs < min_num_errs:
                    min_num_errs = num_errs
                    best_answer_processed = cand_answer_processed

                if not num_errs:
                    break

            _ = compile_and_record_query(best_answer_processed, src_dir, self.query_engine.stringify_prompt(recode_frist_prompt))
            answer_processed = best_answer_processed

            cur_answer += answer_processed
            cur_code = cur_code + "\n" + func_dec + "\n" + func_impl
            prompt = self.update_prompt(cur_code, cur_answer)
            if self.src_lang == "c":
                func_name = (func_dec.split("(")[0].split("\n")[-1].strip().split(" ")[-1])
            else:
                func_name = ""

        logging.info(f" Working on {func_name} function.")
        min_num_errs = 2**32

        recode_second_prompt = copy.deepcopy(prompt)
        recode_second_prompt.constraints = prompt.constraints + self.global_constraints + settings.test_constraints_prompt
        best_answer_processed = ""
        for attempt in range(1, self.transpl_attempt_budget + 1):
            answer = self.query_engine.generate_code(prompt, model_params=self.model_params)

            cand_answer_processed = answer
            comp_out = compile_and_record_query(answer, src_dir, self.query_engine.stringify_prompt(recode_second_prompt))

            parsed_comp_out = parse_error_timepass(comp_out.stderr, self.fname)
            num_errs = parsed_comp_out[-1]

            logging.info(f"\tAttemp {attempt}: {num_errs} errors.")
            if num_errs < min_num_errs:
                min_num_errs = num_errs
                best_answer_processed = cand_answer_processed

            if not num_errs:
                break
        cur_answer += best_answer_processed

        comp_out = compile_and_record_query(cur_answer, src_dir, self.query_engine.stringify_prompt(recode_second_prompt), log_id="trans_decomp")
        answer_processed = cur_answer
        parsed_comp_out = parse_error_timepass(comp_out.stderr, self.fname)

        # FIX
        if parsed_comp_out[-1] and self.comp_fixer is not None:
            if self.comp_fixer.fix_type == "msft":
                logging.info("\tTranspilation FAILED. Attempting to fix compilation errors via LLM.")
                _, _, _, _, init_num_err = parsed_comp_out
                rust_code, fnl_num_err, num_llm_call = self.comp_fixer.fix(answer_processed, parsed_comp_out, src_dir)

                logging.info(f"\t\tNum errors decreased from {init_num_err} to {fnl_num_err}. Fix path was {self.comp_fixer.fix_path}.")
                if not fnl_num_err:
                    os.makedirs(f"{res_dir}", exist_ok=True)

                    subprocess.run(f"cp {self.benchmark_path}/{self.fname}/{self.fname}.json {res_dir}/", shell=True)
                    subprocess.run(f"cp {self.benchmark_path}/{self.fname}/{self.fname}.{self.src_lang} {res_dir}/", shell=True)

                    with open(f"{res_dir}/{self.fname}.rs", "w") as fw:
                        fw.write(rust_code)

                    compiles = True

        elif parsed_comp_out[-1] and self.comp_fixer is None:
            logging.info("\tTranspilation FAILED. No fixer is set.")
        else:
            logging.info("\tTranspilation PASSED.")
            os.makedirs(f"{res_dir}", exist_ok=True)

            subprocess.run(f"cp {self.benchmark_path}/{self.fname}/{self.fname}.json {res_dir}/", shell=True)
            subprocess.run(f"cp {self.benchmark_path}/{self.fname}/{self.fname}.{self.src_lang} {res_dir}/", shell=True)

            with open(f"{res_dir}/{self.fname}.rs", "w") as fw:
                fw.write(answer_processed)

            compiles = True

        # clean project to reduce size
        with cd(f"{src_dir}"):
            subprocess.run("cargo clean", capture_output=True, shell=True)

        return compiles

    def write_src_code_to_res_dir(self, res_dir: str, src_code: str):
        '''
        Write the given source code src_code and corresponding json to the specified result directory res_ir
        Args:
            res_dir:  Result directory
            src_code:  source code
        '''
        with open(f"{res_dir}/{self.fname}.{self.src_lang}", "w") as fw:
            fw.write(src_code)
        subprocess.run(
            f"cp {self.benchmark_path}/{self.fname}/{self.fname}.json {res_dir}/",
            shell=True,
        )

    # @profile
    def transpile_base(self):
        '''
        Convert the specified source code from a language to Rust, and perform save compilation and fix compilation errors
        Returns:
            Compile compilation results
        '''
        compiles = False
        logging.info(f"Now transpiling {self.fname}.")
        with open(f"{self.benchmark_path}/{self.fname}/{self.fname}.{self.src_lang}", "r") as f:
            code = f.read()

        src_dir = f"{self.work_dir}/wspace/"
        res_dir = f"{self.work_dir}/results/"

        prompt = Prompt(
            context=(
                f"You are given a {self.src_lang.capitalize()} code contained in <code> tags."
                + " We need to translate this code to Rust.\n"
                + tag(code, "code")
            ),
            instruction = f"Give me Rust refactoring of above {self.src_lang.capitalize()} code.",
            constraints = self.common_translate_constraints,
            extra_information=self.hint,
        )

        recode_prompt = copy.deepcopy(prompt)
        recode_prompt.constraints = recode_prompt.constraints + self.global_constraints + settings.test_constraints_prompt

        min_num_errs = 2**32
        best_answer_processed = ""
        # Retrieve the source code multiple times, compile and analyze errors to obtain the best one
        for attempt in range(1, self.transpl_attempt_budget + 1):
            cand_answer_processed = self.query_engine.generate_code(prompt, model_params=self.model_params)

            # Create a project, save files, compile source code, and record error messages
            comp_out = compile_and_record_query(cand_answer_processed, src_dir, self.query_engine.stringify_prompt(recode_prompt))
            cand_init_comp_out = parse_error_timepass(comp_out.stderr, self.fname)
            num_errs = cand_init_comp_out[-1]
            logging.info(f"\tAttemp {attempt}: {num_errs} errors.")
            if cand_answer_processed == "":
                continue
            elif num_errs < min_num_errs:
                min_num_errs = num_errs
                best_answer_processed = cand_answer_processed

            if not num_errs:
                break
        if best_answer_processed == "":
            return compiles

        comp_out = compile_and_record_query(best_answer_processed, src_dir, self.query_engine.stringify_prompt(recode_prompt),log_id="trans_base")
        init_comp_out = parse_error_timepass(comp_out.stderr, self.fname)
        answer_processed = best_answer_processed

        # Use the cargo fix command to automatically fix warnings and errors in Rust code
        if self.comp_fixer is not None:
            self.comp_fixer.cargo_fix(src_dir)
            with open(f"{src_dir}/src/lib.rs", "r",encoding='utf-8') as f:
                answer_processed = f.read()

        # If comp_fixer is set and there are errors
        if init_comp_out[-1] and self.comp_fixer is not None:  # if comp_fixer is set and if there is error
            if self.comp_fixer.fix_type == "msft":
                logging.info("\tTranspilation FAILED. Attempting to fix compilation errors via LLM.")
                _, _, _, _, init_num_err = init_comp_out
                rust_code, fnl_num_err, num_llm_call = self.comp_fixer.fix(answer_processed, init_comp_out, src_dir)

                logging.info(f"\t\tNum errors decreased from {init_num_err} to {fnl_num_err}. Fix path was {self.comp_fixer.fix_path}.")

                if not fnl_num_err:
                    os.makedirs(f"{res_dir}", exist_ok=True)
                    self.write_src_code_to_res_dir(res_dir, code)
                    with open(f"{res_dir}/{self.fname}.rs", "w") as fw:
                        fw.write(rust_code)

                    compiles = True

        elif init_comp_out[-1] and self.comp_fixer is None:
            logging.info("\tTranspilation FAILED. No fixer is set.")

        else:
            logging.info("\tTranspilation PASSED.")
            os.makedirs(f"{res_dir}", exist_ok=True)
            self.write_src_code_to_res_dir(res_dir, code)
            shutil.copy2(f"{src_dir}/src/lib.rs", f"{res_dir}/{self.fname}.rs")

            compiles = True

        # clean project to reduce size
        with cd(f"{src_dir}"):
            subprocess.run("cargo clean", capture_output=True, shell=True)

        return compiles
