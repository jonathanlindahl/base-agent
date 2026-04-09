use crate::llm::call_llm;
use crate::tools::{Calculator, Tool};

pub struct Agent {
    tools: Vec<Box<dyn Tool>>,
}

impl Agent {
    pub fn new() -> Self {
        Self {
            tools: vec![Box::new(Calculator {})],
        }
    }

    pub async fn run(&mut self, input: &str) -> anyhow::Result<String> {
        let prompt = format!(
            r#"
You are an AI agent.

You must follow instructions EXACTLY.

You can either:
1. Respond directly
2. Use a tool

You must always prioritize using a tool if you can.

If you use a tool, you MUST response EXACTLY in this format:

TOOL: <tool_name>
INPUT: <input>

Rules:
- Do not add ANY explanation or extra text.
- NEVER create extra questions
- NEVER create user inputs
- NEVER repeat the same questions

Available tools:
- calculator

User: {}
        "#,
            input
        );

        let first_response = call_llm(&prompt).await?;

        println!("\nLLM says:\n{}", first_response);

        if first_response.contains("TOOL:") {
            let tool_name = extract(&first_response, "TOOL:");
            let tool_input = extract(&first_response, "INPUT:");

            let tool = self
                .tools
                .iter()
                .find(|t| t.name() == tool_name)
                .ok_or_else(|| anyhow::anyhow!("Tool not found"))?;

            let result = tool.execute(&tool_input)?;

            println!("\nTool result: {}", result);

            let second_prompt = format!("Tool result: {}\n\nGive final answer to user.", result);

            let final_response = call_llm(&second_prompt).await?;
            return Ok(final_response);
        }
        Ok(first_response)
    }
}

fn extract(text: &str, key: &str) -> String {
    text.lines()
        .find(|line| line.trim_start().starts_with(key))
        .map(|line| line.replace(key, "").trim().to_string())
        .unwrap_or_default()
}
