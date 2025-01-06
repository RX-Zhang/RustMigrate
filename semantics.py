'''
# Code for implementing feedback strategy
'''
import copy
from dataclasses import dataclass
from typing import Any, Optional, List, Tuple, Union
import logging
import json
import random
import tempfile
import functools

from overrides import override
from itertools import starmap
from subprocess import CalledProcessError
import llms
from llms import Prompt, QueryEngine
import settings

from utils import (compile_and_record_query,parse_error_coarse,tag)
from settings import Options
import oracle

class Enhancement:
    def __init__(self, replay_dir: str, positive_examples: str, negative_examples: str):
        '''
        Args:
            replay_dir:  The path to the replay directory.
            positive_examples:  Positive example, in string form.
            negative_examples:  Negative example, in string form.
        '''
        # TODO remove this constant?
        N_EXAMPLES = 10
        # Grouping negative examples
        cov_to_ce = oracle.group_examples_by_coverage(replay_dir, negative_examples, N_EXAMPLES)
        # Randomly select a group
        _, ce_group = random.choice(list(cov_to_ce.items()))
        #  len(ce_group) <= N_EXAMPLES
        if len(ce_group) > N_EXAMPLES:
            ce_group = random.sample(ce_group, N_EXAMPLES)
        else:
            ce_group = ce_group

        self.ce_group = ce_group

    def enhancement(
        self, context: str,
            textual_examples: str,
            query_engine: QueryEngine
    ) -> str:
        return ""


class LLMExplain(Enhancement):
    def __init__(self, replay_dir: str, positive_examples: str, negative_examples: str):
        super().__init__(replay_dir, positive_examples, negative_examples)

    @override
    def enhancement(
        self, context: str, textual_examples: str, query_engine: QueryEngine
    ) -> str:
        '''
        Enhance the question through textual_examples and get the answer from LLM.
        Args:
            context: Textual question
            textual_examples: Textual representation of input/output examples.
            query_engine: Query engine used to send queries to the LLM and obtain answers.
        Returns:
            Information returned by LLM
        '''
        logging.info("Enhancing prompt with LLM-Based root cause explanations.")

        explain_prompt = Prompt(
            context=context,
            instruction="Tell me the root cause of the issue and how to fix it in the Rust code.", # Tell me the root cause of the problem and how to fix it in Rust code
            constraints=[
                "Just provide the reason for the error and repair suggestions, no further information is needed."
            ],
            extra_information=(
                "A set of input/output example(s) contained in <testcases> tag is given below."   # Below is a set of input/output examples included in the<testcases>tag.
                + tag(textual_examples, "testcases")
            ),
        )
        # Using LLM to obtain the cause of errors
        answer = query_engine.query(explain_prompt)

        enhancement = (
            "\nHere is a possible explanation and solution in <answers>, please use it as reference information when solving problems."
            + tag(answer, "answers")
        )

        return enhancement


@functools.total_ordering # Simplified the implementation of comparison operators for custom classes and automatically generated other comparison operators
class Candidate:
    def __init__(
        self,
        rust_code: str,
        positive_examples: str,
        negative_examples: str,
        extra: Union[None, Enhancement, Tuple[str, List[Tuple[str, str]]]],
    ) -> None:
        self.rust_code = rust_code
        self.positive_examples = positive_examples
        self.negative_examples = negative_examples
        ne = json.loads(self.negative_examples)
        pe = json.loads(self.positive_examples)
        self.score = len(pe) / (len(pe) + len(ne))
        self.extra = extra # Enhancement or LLMExplain

    def hint(self, n_examples: int) -> str:
        '''
        Generate a prompt containing a certain number of positive and negative examples
        Args:
            n_examples:  The total number of examples to be included in the prompt
        Returns:
            hint : string
        '''
        n_negatives = n_examples
        logging.info(f"Hinted with  {n_negatives} negative examples")

        negative_examples = json.loads(self.negative_examples)
        #   len(?) <= n_?
        if len(negative_examples) > n_negatives:
            negative_examples = random.sample(negative_examples, n_negatives)

        examples = list(
            starmap(  # Apply the specified function to each element in the iterable object
                lambda idx, example: f"Example_{idx}:\n{{ {textual_example(example)} }}",
                enumerate(negative_examples),
            )
        )  #   { Example 0:\n Arguments:\n  Argument 0: {..}\n  Argument 1: {..}\n Expected Output: {..}\n ...}
        # Please consider the following input/output examples included in the<testcases>tag:
        preamble = "Consider the following input/output examples included in the<testcases>tag, where Expected Output and Actual Output are the output results of C and Rust functions, respectively, under the same input args\n"

        hint = preamble + tag("\n".join(examples), "testcases")

        return hint

    def prompt(
        self,
        query_engine: QueryEngine,
        src_code: str,
        src_lang: str,
        n_examples: int,
        history: List[Tuple[str, str]] = [],
    ) -> Prompt:
        if self.ok:
            assert not self.extra
            raise RuntimeError("Ok candidate should not have this.")
        
        if n_examples == 0:
            assert len(history) == 0, "CAPR requires counter examples"

        # Counter examples enhancement
        if len(self.extra.ce_group) > n_examples:
            ce_group = random.sample(self.extra.ce_group, n_examples)
        else:
            ce_group = self.extra.ce_group
        # Obtain sample information
        textual_examples = list_examples(ce_group)

        if len(history) > 0:
            return Prompt(
                # context="In the previous modifications, the program still had incorrect input and output in <testcase>:\n" + tag(textual_examples, "testcases"),
                context="In the previous modifications, the program still had incorrect input and output.",
                instruction="Please modify the given code again to obtain the expected output of the given test input.",
                constraints=[
                    "Use only safe Rust.",
                    "Don't use raw pointers.",
                    "Use box pointer whenever possible. Box pointers are preferable to other alternatives.",
                    "Try not to use Traits if possible. I would not like to have Traits in resulting Rust code.",
                    "Try not to use custom Generics if possible.",
                ],
                history=history
            )

        context = (
            f"\nYou are given a {src_lang} code contained in the following <code> tag"
            + tag(src_code, "code")
            + "You will also receive a compiled Rust translation included in the <code> tag that does not provide the expected output for certain inputs."
            + tag(self.rust_code, "code")
        )

        extra_information: str
        if n_examples > 0:
            enhancement = self.extra.enhancement(context, textual_examples, query_engine) # ""
            extra_information = (
                "A set of input/output example(s) contained in <testcases> tag is given below."
                + tag(textual_examples, "testcases")
                + enhancement
            )
        else:
            extra_information = ""

        prompt = Prompt(
            context=context,
            instruction="Make changes in the given code to obtain expected outputs for given test inputs.",
            constraints=[
                "Use only safe Rust.",
                "Don't use raw pointers.",
                "Use box pointer whenever possible. Box pointers are preferable to other alternatives.",
                "Try not to use Traits if possible. I would not like to have Traits in resulting Rust code.",
                "Try not to use custom Generics if possible.",
            ],
            extra_information=extra_information,
        )

        return prompt

    @property
    def ok(self) -> bool:
        return self.score == 1

    def __eq__(self, other):
        if not isinstance(other, Candidate):
            raise NotImplementedError
        return self.score == other.score

    def __lt__(self, other):
        if not isinstance(other, Candidate):
            raise NotImplementedError
        return self.score < other.score


class CandidateFactory:
    def __init__(
        self,
        src_code: str,
        src_code_json: str,
        language: str,
        submodule_name: str,
        sem_fix: str,   # options.sem_fix
        workspace_name: str
    ) -> None:
        self.src_code = src_code
        self.src_code_json = src_code_json
        self.language = language
        self.submodule_name = submodule_name
        self.workspace_name = workspace_name
        if sem_fix == "base":
            Extra = Enhancement
        elif sem_fix == "llm-explain":
            Extra = LLMExplain
        else:
            raise NotImplementedError
        self.Extra = Extra

    @property
    def preamble(self) -> str:
        '''
            Return prompt string
        '''
        ret = (
            f"\n\nYou are given a {self.language} code contained in the following <code> tag\n"
            + tag(self.src_code, "code")
            + "\n"
            "You are also given a plausible Rust translation contained in <code> tag that does not provide expected outputs for certain inputs. "
            "A set of example(s) contained in <testcases> tag is given after the code.\n"
        )
        return ret

    def debug_candidate(self, candidate: Candidate) -> None:
        '''
        Build the project, execute the Rust script, and revalidate the target
        Args:
            candidate:  Candidate examples
        '''
        with tempfile.TemporaryDirectory(ignore_cleanup_errors=True) as tmp_dir:
            src_dir = tmp_dir
            # Save code to temporary directory
            with open(src_dir + f"/{self.submodule_name}.{self.language}","w") as f:
                f.write(self.src_code)
            with open(src_dir + f"/{self.submodule_name}.json","w") as f:
                f.write(self.src_code_json)
            with open(src_dir + f"/{self.submodule_name}.rs", "w") as f:
                f.write(candidate.rust_code)

            workspace = tmp_dir + "/replay"
            try:
                oracle.instrument(
                    self.language, src_dir, self.submodule_name, workspace
                )
            except:
                raise AssertionError("Instrumentation should succeed")
            validation_result = oracle.soft_verify(
                workspace,
                self.submodule_name,
                candidate.positive_examples,
                candidate.negative_examples,
            )
            # should produce result_process
            assert validation_result, "Soft verification should succeed"

            positive_examples, negative_examples = validation_result

            assert (
                positive_examples == candidate.positive_examples
            ), "Incorrect positive examples"
            assert (
                negative_examples == candidate.negative_examples
            ), "Incorrect negative examples"

    def construct_candidate(
        self,
        rust_code: str,
        positive_examples: Optional[str] = None,
        negative_examples: Optional[str] = None,
    ) -> Optional[Candidate]:
        '''
            Generate fuzz test code, perform fuzz testing, obtain positive and negative examples, and then construct candidate objects
        Args:
            rust_code: Rust code
            positive_examples:
            negative_examples:
        Returns:
            candidate
        '''
        with tempfile.TemporaryDirectory(ignore_cleanup_errors=True) as tmp_dir:
            src_dir = tmp_dir
            # 保存代码(C, json, Rust)到临时目录
            with open(src_dir + f"/{self.submodule_name}.{self.language}","w") as f:
                f.write(self.src_code)
            with open(src_dir + f"/{self.submodule_name}.json","w") as f:
                f.write(self.src_code_json)
            with open(src_dir + f"/{self.submodule_name}.rs", "w") as f:
                f.write(rust_code)

            workspace = tmp_dir + "/replay"
            try:
                # Call florine to automatically generate bolero test rust code
                oracle.instrument(self.language, src_dir, self.submodule_name, workspace)
            except CalledProcessError:
                logging.info("Failed to instrument candidate.")
                return None

            requires_verification: bool = not positive_examples or not negative_examples
            validation_result: Optional[Tuple[str, str]]
            if requires_verification:
                # First validation, perform fuzz testing, return positive and negative examples
                validation_result = oracle.verify(workspace, self.submodule_name, self.workspace_name) # workspace = tmp_dir/replay
            else: # When providing feedback, there are both positive and negative examples
                validation_result = oracle.soft_verify(workspace, self.submodule_name, positive_examples, negative_examples)

            if not validation_result:
                logging.info("Failed to generate oracle.")
                return None
            positive_examples, negative_examples = validation_result
            candidate: Candidate
            try:
                # create candidate
                candidate = Candidate(rust_code, positive_examples, negative_examples, None)
            except json.decoder.JSONDecodeError:
                # occasionally our instrumentor cannot handle some json data
                return None
            if not candidate.ok:
                # Enhancement or LLMExplain
                candidate.extra = self.Extra(workspace, positive_examples, negative_examples)
            elif requires_verification:
                candidate.extra = oracle.compute_coverage_by_libfuzzer_corpus(workspace)
            return candidate


@dataclass(eq=False, repr=False)
class SemanticsStrategy:
    restart_idx: int
    factory: CandidateFactory
    options: Options
    query_engine: QueryEngine
    global_constraints: list[str]
    budget: int

    def optimize(self, candidate: Candidate) -> Candidate:
        '''
        Optimize candidate solutions iteratively until specific conditions are met or budget constraints are reached.
        In each round of optimization, it will attempt to fix the problems in the current candidate solution and evaluate whether the repaired candidate solution is better than the current best candidate solution.
        If the condition is met (such as the OK attribute of the candidate solution being true), return the optimized candidate solution; Otherwise, continue iterating until the budget limit is reached.
        '''
        round_idx = 0
        history: List[Tuple[str, str]] = []
        while self.budget > 0:
            logging.info(f"Starting the {round_idx}-th round of fixing. Beam size = 1.")
            print(f"history length = {len(history)}")

            # Use LLM to fix the code. If the compilation is correct, build a new candidate
            new_candidates = self.fix(candidate, history)

            new_candidates.sort(reverse=True)
            logging.info(f"{len(new_candidates)} many (potentially new) candidates expanded. Highest score = {new_candidates[0].score}")

            assert len(new_candidates) > 0
            candidate = new_candidates[0]
            if candidate.ok:
                return candidate

            self.budget -= 1
            round_idx += 1

        return candidate

    def fix(self, candidate: Candidate, history: List[Tuple[str, str]]) -> List[Candidate]:
        '''
        Using LLM to Fix Code Semantic Inconsistencies
        Args:
            candidate:  Candidate object
            history:  historical information
        Returns:
            candidate:  Fixed candidate objects
        '''
        prompt = candidate.prompt(
            self.query_engine,
            self.factory.src_code,
            self.factory.language,
            self.options.n_prompt_examples,
            history=history[:(self.options.conversation_window_size * 2)],
        )
        REP_THOLD = 5
        trial = 0
        new_rust_code: str

        recode_prompt = copy.deepcopy(prompt)
        recode_prompt.constraints = prompt.constraints + self.global_constraints + settings.test_constraints_prompt
        while trial < REP_THOLD:

            new_rust_code = self.query_engine.generate_code(prompt)
            comp_out = compile_and_record_query(new_rust_code, self.src_dir, self.query_engine.stringify_prompt(recode_prompt), log_id=f"sem_fix_{self.restart_idx}_{self.budget}")
            comp_out = parse_error_coarse(comp_out.stderr)
            if not len(comp_out[0]):
                break
            logging.info("Fixed code does not compile. Giving it another try.")
            trial += 1

        if len(comp_out[0]):
            # TODO
            logging.info("Could not find a fix that compiles. Giving up.")
            return [candidate]

        new_candidate = self.factory.construct_candidate(new_rust_code, candidate.positive_examples, candidate.negative_examples)

        if not new_candidate or new_candidate <= candidate:
            logging.info("Found candidate of bad quality. Giving up.")
            return [candidate]
        
        if self.options.conversation:
            history.append((llms.USER, str(prompt)))
            history.append((llms.ASSISTANT, new_rust_code))

        return [new_candidate]

    @property
    def src_dir(self) -> str:
        return f"{self.options.work_dir}/wspace"

def list_examples(negative_examples: List[Any]) -> str:
    '''
    Generate a string that lists detailed information about all negative examples, including their input parameters, expected output, and actual output.
    Args:
        negative_examples:  Negative example
    Returns:

    '''
    RETURN_VOID = "\"Program execution successful, no return value\""
    examples_list = ""
    for ce_idx, s_ce in enumerate(negative_examples):
        Expected_Output: str
        Actual_Output: str
        expect_return_results = ""
        actual_return_results = ""
        if s_ce["actual"] == "ExecutionFailure":
            Actual_Output = "Under this input parameter, the program crashes"
        else:
            output = s_ce["actual"]["ExecutionSuccess"]
            if isinstance(output, str):
                output_part = output.split(':', 1)
                actual_return_results = output_part[0]
                if len(output_part)>1:
                    Actual_Output = simplify_data(json.loads(output_part[1]))
                    Actual_Output = RETURN_VOID if Actual_Output is None else Actual_Output

        if s_ce["expected"] == "ExecutionFailure":
            Expected_Output = "Under this input parameter, the program crashes"
        else:
            output = s_ce["expected"]["ExecutionSuccess"]
            if isinstance(output, str):
                output_part = output.split(':', 1)
                expect_return_results = output_part[0]
                if len(output_part)>1:
                    Expected_Output = simplify_data(json.loads(output_part[1]))
                    Expected_Output = RETURN_VOID if Expected_Output is None else Expected_Output

        arguments = []
        for arg_idx, arg in enumerate(s_ce["args"]):
            arg = json.loads(arg)
            arg = simplify_data(arg)
            arguments.append(arg)

        if (s_ce["expected"] != "ExecutionFailure" and s_ce["actual"] != "ExecutionFailure") and (
                expect_return_results != "output" and actual_return_results != "output"):
            examples_list = (
                examples_list + f"\n Example_{ce_idx}:\n "
                                f"{{"
                                f"\"args\":{arguments},\n"
                                f"\"Expected Output\":{RETURN_VOID},\n"
                                f"\"Expected {expect_return_results}\":{Expected_Output},\n"
                                f"\"Actual Output\":{RETURN_VOID},\n"
                                f"\"Actual {actual_return_results}\":{Actual_Output}\n"
                                f"}}"
            )
        else:
            examples_list = (
                    examples_list + f"\n Example_{ce_idx}:\n "
                                    f"{{"
                                    f"\"args\":{arguments},\n"
                                    f"\"Expected Output\":{Expected_Output},\n"
                                    f"\"Actual Output\":{Actual_Output}\n"
                                    f"}}"
            )

    return examples_list


def simplify_data(json_data):
    '''
    Simplify the representation of JSON data
    eg: input: {"key1": [1, 2, 3, 4, 5, 6, 7]}
        output: {"key1": [1, 2, 3, 4, "... and 2 other elements"]}
    '''
    MAX_ARRAY_LENGTH = 5
    if isinstance(json_data, dict):
        return {key: simplify_data(value) for key, value in json_data.items()}
    elif isinstance(json_data, list):
        if len(json_data) > MAX_ARRAY_LENGTH:
            n_removed = len(json_data) - MAX_ARRAY_LENGTH
            return [simplify_data(value) for value in json_data[:MAX_ARRAY_LENGTH]] + [
                f"... and {n_removed} other elements"
            ]
        else:
            return [simplify_data(value) for value in json_data]

    return json_data


def textual_example(example: Any) -> str:
    '''
    Convert an example data into a string that describes the example.
    Args:
        example:

    Returns: (str)
        Argument {arg_idx}: {arg}
        Expected Output: {output}

    '''
    RETURN_VOID = "\"Program execution successful, no return value\""

    Expected_Output: str
    Actual_Output: str
    expect_return_results = ""
    actual_return_results = ""
    # expected
    try:
        if example["expected"] == "ExecutionFailure":
            Expected_Output = "Under this input parameter, the program crashes"
        else:
            output = example["expected"]["ExecutionSuccess"]
            if isinstance(output, str):
                output_part = output.split(':',1)
                expect_return_results = output_part[0]
                if len(output_part)>1:
                    Expected_Output = simplify_data(json.loads(output_part[1]))
                    Expected_Output = RETURN_VOID if Expected_Output is None else Expected_Output

            else:
                Expected_Output = example["expected"]
    except KeyError:
        raise  ValueError("KeyError")

    # actual
    if example["actual"] == "ExecutionFailure":
        Actual_Output = "Under this input parameter, the program crashes"
    else:
        output = example["actual"]["ExecutionSuccess"]
        if isinstance(output, str):
            output_part = output.split(':',1)
            actual_return_results = output_part[0]
            if len(output_part) > 1:
                Actual_Output = simplify_data(json.loads(output_part[1]))
                Actual_Output = RETURN_VOID if Actual_Output is None else Actual_Output

        else:
            Actual_Output = example["actual"]

    arguments = []
    for arg_idx, arg in enumerate(example["args"]):
        arg = json.loads(arg)
        arg = simplify_data(arg)
        arguments.append(arg)


    if (example["expected"] != "ExecutionFailure" and example["actual"] != "ExecutionFailure" ) and (expect_return_results != "output" and actual_return_results != "output"):
        return (f"\"args\":{arguments},\n"
                f"\"Expected Output\":{RETURN_VOID},\n"
                f"\"Expected {expect_return_results}\":{Expected_Output},\n"
                f"\"Actual Output\":{RETURN_VOID},\n"
                f"\"Actual {actual_return_results}\":{Actual_Output}\n")
    else:
        return (f"\"args\":{arguments},\n"
                f"\"Expected Output\":{Expected_Output},\n"
                f"\"Actual Output\":{Actual_Output}\n")