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
