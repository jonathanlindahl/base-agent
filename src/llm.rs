use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct RequestBody {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct Response {
    response: String,
}

pub async fn call_llm(prompt: &str) -> anyhow::Result<String> {
    let client = reqwest::Client::new();
    let body = RequestBody {
        model: "tinyllama".to_string(),
        prompt: prompt.to_string(),
        stream: false,
    };

    let res = client
        .post("http://localhost:11434/api/generate")
        .json(&body)
        .send()
        .await?;

    let json: Response = res.json().await?;

    Ok(json.response)
}
