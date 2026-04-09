pub trait Tool {
    fn name(&self) -> &str;
    fn execute(&self, input: &str) -> anyhow::Result<String>;
}

pub struct Calculator;

impl Tool for Calculator {
    fn name(&self) -> &str {
        "calculator"
    }

    fn execute(&self, input: &str) -> anyhow::Result<String> {
        let result = meval::eval_str(input)?;
        Ok(result.to_string())
    }
}
