# Machine-Learning-AI-Language-Model-Tutor

This repository contains a Rust application that demonstrates how to use OpenAI's GPT-3 API to create an AI-powered language model tutor. The application sends a prompt to the GPT-3 API, which responds with an informative and relevant answer.

# Requirements
Rust programming language (https://www.rust-lang.org/tools/install)

Reqwest crate (https://crates.io/crates/reqwest)

Serde JSON crate (https://crates.io/crates/serde_json)

# Setup

1. Clone this repository to your local machine.

2. Navigate to the project directory.
``` bash
cd machine-learning-ai-language-model-tutor
```

3. Set up an environment variable OPENAI_API_KEY with your OpenAI API key. Make sure to replace <your_api_key> with your actual API key.

`export OPENAI_API_KEY=<your_api_key>`

4. Build the project.

`cargo build`

5. Run the application.

`cargo run`

The application will send a prompt to the GPT-3 API and print the AI-generated response in the terminal.