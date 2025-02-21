from dataclasses import dataclass

test_constraints_prompt = [
    "Try not to use Traits if possible. I would not like to have Traits in resulting Rust code.",
    "Try not to use custom Generics if possible.",
    "Use standalone functions rather than struct methods.",
    "Just provide me with the translated Rust function, without any additional explanations or a main function calling this function.",
]


#---------------------------------------------------------------------------------------------
#  my_method | fallback_opt |   comp_fix  |      sem_fix     |  hinted  |    conversation    |
#------------|--------------|-------------|------------------|----------|--------------------|
# BaseRepair |      fix     |   msft/no   | base/llm-explain |     -    |        False       |
#    CAPR    |      fix     |   msft/no   | base/llm-explain |     -    |         True       |
#   Hinted   |    restart   |   msft/no   | base/llm-explain |   True   |          -         |
#   Restart  |    restart   |   msft/no   | base/llm-explain |   False  |          -         |
#---------------------------------------------------------------------------------------------

@dataclass(frozen=True)
class Options:
    benchmark_name: str   # choices = ["libopenaptx", "opl"]
    submodule_name: str  # Name of each module under the benchmark
    model: str  # choices=["claude3", "gpt4", "mistral" ,"gemini"]  "claude2"
    feedback_strategy: str  # choices=["BaseRepair", "CAPR", "Hinted" ,"Restart"]

    restart_budget: int = 3  # default = 3
    fix_budget: int = 5  # default = 5
    comp_fix_attempt_budget: int = 3  # default = 3
    n_prompt_examples: int = 2 # Number of examples in feedback information
    transpl_attempt_budget: int = 3  # The number of times LLM translation is called default = 3
    timeout: int = 3

    fallback_opt: str = "restart"  # choices = ["fix", "restart", "param-search", "simplify"]
    language: str = "c"
    comp_fix: str = "msft"  # choices = [ "msft", "no"]
    sem_fix: str = "base"  # choices=["base","llm-explain"]  #default "base"  -> Extra
    hinted: bool = False  # default = False
    conversation: bool = False  # default = False
    conversation_window_size: int = 2
    initial_temperature: float = 0.2


    @property
    def work_dir(self) -> str:
        return (
            f"transpilations/{self.language}/{self.benchmark_name}/{self.model}/{self.feedback_strategy}/fallback_opt-{self.fallback_opt}/temperature-{self.initial_temperature}/"
            f"comp_fix-{self.comp_fix}/{self.restart_settings_path}{self.fix_settings_path}"
            f"{self.submodule_name}"
        )

    @property
    def res_dir(self) -> str:
        return f"{self.work_dir}/results"

    @property
    def sem_fix_m(self) -> str:
        return "sem-" + self.sem_fix + "-fix"

    @property
    def fix_settings_path(self) -> str:
        if self.fallback_opt == "fix":
            conversation_setting: str
            if not self.conversation:
                conversation_setting = "False"
            else:
                conversation_setting = "True"
            if self.n_prompt_examples == 0:
                assert not self.conversation, "CAPR requires counter examples"
            return f"{self.sem_fix_m}/fix_budget-{self.fix_budget}/conversation-{conversation_setting}/"
        else:
            return "/"

    @property
    def restart_settings_path(self) -> str:
        return f"restart-{self.restart_budget}/hinted-{self.hinted}/"
