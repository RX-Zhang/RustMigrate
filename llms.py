'''
# Handling code with correctly formatted prompts for different LLMs
'''
import time

import anthropic
import boto3
import botocore
import logging
from abc import abstractmethod
from dataclasses import dataclass, field
import json
import requests
from mistralai.client import MistralClient
from mistralai.models.chat_completion import ChatMessage
import re
from typing import Any, List, Dict, Tuple, Union
import google.generativeai
import settings

from overrides import override
from tenacity import (
    retry,
    wait_random_exponential,
    stop_after_delay,
    retry_if_exception_type,
)

from utils import tag

USER = "USER"
ASSISTANT = "ASSISTANT"

MAX_TOKEN: int = 8192

@dataclass
class Prompt:
    """
    Structured representation of the prompt
    Args:
        history (List[Tuple[str, str]]): A conversation consisting of a list of role and content pairs
        preamble (str): A fixed preamble used for the response.
    """
    # Static shared variables
    context: str = ""
    instruction: str = ""
    constraints: List[str] = field(default_factory=list)
    extra_information: str = ""

    history: List[Tuple[str, str]] = field(default_factory=list)
    preamble: str = ""

    def __str__(self) -> str:
        constraints_str = ""
        for c_id, constraint in enumerate(self.constraints):
            constraints_str = (
                constraints_str + "\n\t" + str(c_id + 1) + ". " + constraint
            )
        return f"""{self.context}
                {self.instruction}
                Here are some constraints contained in <list> tags that you should respect:
                {tag(constraints_str, "list")}
                {self.extra_information}
                """

class QueryError(Exception):
    """
    A wrapper around all sorts of errors thrown by LLMs
    """

    pass

class QueryEngine:
    def __init__(self, global_constraints: List[str]) -> None:
        self.global_constraints = global_constraints

    @abstractmethod
    def raw_query(
        self,
        prompt: Union[str, Prompt],
        model_params: Dict[str, Any],
    ) -> str: ...

    @retry(
        reraise=True,
        retry=retry_if_exception_type(QueryError),
        wait=wait_random_exponential(multiplier=1, max=120),
        stop=stop_after_delay(900),
    )
    def query(
        self,
        prompt: Prompt,
        model_params: Dict[str, Any] = {"temperature": 0.2},
    ) -> str:

        return self.raw_query(prompt, model_params)

    def stringify_prompt(self, prompt: Prompt) -> str:
        """
        Convert the Prompt object to a string in a specific format
        """
        messages = self.messages(prompt) # list[Dict(role:content,role:content)]
        prompt_str = ""
        for message in messages:
            role = message["role"]
            content = message["content"]
            prompt_str += f"{role}:\n{content}\n"

        return prompt_str

    def generate_code(
        self, prompt: Prompt,
        model_params: Dict[str, Any] = {"temperature": 0.2}
    ) -> str:
        '''
        Use LLM to generate code
        Args:
            prompt: The prompt message
            model_params: The model parameters
        Returns:
            The generated code block
        '''
        constrained_prompt = Prompt(
            context=prompt.context,
            instruction=prompt.instruction,
            constraints=prompt.constraints
            + self.global_constraints
            + settings.test_constraints_prompt,
            extra_information=prompt.extra_information,
            preamble=prompt.preamble,
            history=prompt.history,
        )
        response = self.query(constrained_prompt, model_params)
        return QueryEngine.extract(response)

    @staticmethod
    def extract(response: str) -> str:
        tagged_block = re.search(r"<code>(?P<code>[\s\S]*)</code>", response) #The function searches for the first code block that matches the<code></code>tag.
        if tagged_block: # tagged_block
            return tagged_block["code"]
        backticked_block = re.search(r"```(rust)?(?P<code>[\s\S]*)```", response)
        if backticked_block:
            return backticked_block["code"]

        return response

    def messages(
        self,
        prompt: Union[str, Prompt], # Prompt can be str or prompt
    ) -> List[Dict[str, str]]:
        '''
        Record log messages in a fixed format
        Args:
            prompt: 提示(str或Prompt)

        Returns:
            messages: {"role": "user/assistant", "content": content}
        '''
        if isinstance(prompt, str):
            messages = [
                {"role": "user", "content": prompt},
            ]
        else:
            messages = []
            for content in prompt.history:
                role, content = content
                if role == USER:
                    messages.append({"role": "user", "content": content})
                elif role == ASSISTANT:
                    messages.append({"role": "assistant", "content": content})
                else:
                    raise ValueError(f"Unidentified role: {role}")

            messages.append({"role": "user", "content": str(prompt)})

            if prompt.preamble:
                messages.append({"role": "assistant", "content": prompt.preamble.rstrip()})
        return messages


class Claude3(QueryEngine):
    def __init__(self, global_constraints: List[str]) -> None:
        super().__init__(global_constraints)
        self.model = "claude-3-sonnet-20240229"
        self.url = 'https:// '
        self.headers = {"content-type": "application/json",
                        "Authorization": "Bearer sk- "}

    @override
    def raw_query(
        self,
        prompt: Union[str, Prompt],
        model_params: Dict[str, Any],
    ) -> str:
        result = ""
        try:
            data = {
                "messages": self.messages(prompt),
                "model": self.model,
                "temperature": model_params["temperature"],
            }
            data = json.dumps(data)
            response = requests.post(self.url, data=data, headers=self.headers)

            if response.status_code == 200:
                try:
                    response_json = response.json()
                    result = response_json["choices"][0]["message"]["content"]
                except ValueError as e:
                    raise QueryError(e)
        except Exception as e:
            raise QueryError(e)

        logging.info(f"A query to Anthropic Claude3 is made with model paramters as follows: {str(model_params)}")

        return result


class Mistral(QueryEngine):
    def __init__(self, global_constraints: List[str]) -> None:
        super().__init__(global_constraints)
        self.model = "open-mixtral-8x7b"
        self.url = ''
        self.headers = {"content-type": "application/json",
                        "Authorization": "Bearer "}

    @override
    def raw_query(
        self,
        prompt: Union[str, Prompt],
        model_params: Dict[str, Any],
    ) -> str:
        result = ""
        try:
            data = {
                "messages": self.messages(prompt),
                "model": self.model,
                "temperature": model_params["temperature"],
            }
            data = json.dumps(data)
            response = requests.post(self.url, data=data, headers=self.headers)

            if response.status_code == 200:
                try:
                    response_json = response.json()
                    result = response_json["choices"][0]["message"]["content"]
                except ValueError as e:
                    raise QueryError(e)
            else:
                print("llm feedback failed")
                exit(1)
        except Exception as e:
            raise QueryError(e)

        logging.info(f"A query to Mistral is made with model paramters as follows: {str(model_params)}")

        return result


class GPT4(QueryEngine):
    def __init__(self, global_constraints: List[str]) -> None:
        super().__init__(global_constraints)
        self.model = "gpt-4-turbo-preview"
        self.url = ''
        self.headers = {"content-type": "application/json",
                        "Authorization": "Bearer "}


    @override
    def raw_query(
        self,
        prompt: Union[str, Prompt],
        model_params: Dict[str, Any],
    ) -> str:
        result = ""
        try:
            data = {
                "messages": self.messages(prompt),
                "model": self.model,
                "temperature": model_params["temperature"],
            }
            data = json.dumps(data)
            response = requests.post(self.url, data=data, headers=self.headers, timeout=30)

            if response.status_code == 200:
                try:
                    response_json = response.json()
                    # time.sleep(10)
                    result = response_json["choices"][0]["message"]["content"]
                except ValueError as e:
                    raise QueryError(e)

        except Exception as e:
            raise QueryError(e)

        logging.info(f"A query to GPT4 is made with model paramters as follows: {str(model_params)}")

        return result


class Gemini(QueryEngine):
    def __init__(self, global_constraints: List[str]) -> None:
        super().__init__(global_constraints)
        self.model = "gemini-1.5-pro-exp-0827"
        self.url = ''
        self.headers = {"content-type": "application/json",
                        "Authorization": "Bearer "}

    @override
    def raw_query(
        self,
        prompt: Union[str, Prompt],
        model_params: Dict[str, Any],
    ) -> str:
        result = ""
        try:
            data = {
                "messages": self.messages(prompt),
                "model": self.model,
                "temperature": model_params["temperature"],
            }
            data = json.dumps(data)
            response = requests.post(self.url, data=data, headers=self.headers)

            if response.status_code == 200:
                try:
                    # 解析JSON响应
                    response_json = response.json()
                    result = response_json["choices"][0]["message"]["content"]
                except ValueError as e:
                    raise QueryError(e)
        except Exception as e:
            raise QueryError(e)
        logging.info(f"A query to Gemini is made with model paramters as follows: {str(model_params)}")
        return result

class QueryEngineFactory:
    @staticmethod
    def create_engine(model: str, global_constraints: List[str] = []) -> QueryEngine:
        match model:
            case "claude3":
                return Claude3(global_constraints)
            case "gpt4":
                return GPT4(global_constraints)
            case "mistral":
                return Mistral(global_constraints)
            case "gemini":
                return Gemini(global_constraints)
            case _:
                raise ValueError(f"Unknown model: {model}")
