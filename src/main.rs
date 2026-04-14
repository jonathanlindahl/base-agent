mod agent;
mod llm;
mod tools;

use agent::Agent;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut agent = Agent::new();

    println!("Ask something:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let response = agent.run(input.trim()).await?;

    println!("\nFinal answer:\n{}", response);

    Ok(())
}
