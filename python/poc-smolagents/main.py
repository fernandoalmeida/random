import os

from dotenv import load_dotenv
from huggingface_hub import login
from smolagents import CodeAgent, TransformersModel


def main():
    load_dotenv()
    huggingface_token = os.getenv("HUGGINGFACE_HUB_TOKEN")
    login(token=huggingface_token)

    model_id = "deepseek-ai/DeepSeek-R1-Distill-Qwen-1.5B"
    model = TransformersModel(model_id=model_id)
    agent = CodeAgent(tools=[], model=model, add_base_tools=True)

    agent.run("Could you give me the 4th number in the Fibbonacci sequence?")

if __name__ == "__main__":
    main()
