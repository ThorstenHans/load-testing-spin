use serde::Serialize;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

#[derive(Debug, Serialize)]
pub struct Sample {
    value: String,
    key: i8,
    active: bool,
}

impl Default for Sample {
    fn default() -> Self {
        Self {
            value: "What is the meaning of life?".to_string(),
            key: 42,
            active: true,
        }
    }
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_scenario(_: Request) -> anyhow::Result<impl IntoResponse> {
    let payload = serde_json::to_vec(&Sample::default())?;
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(payload)
        .build())
}
