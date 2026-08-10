---
id: fixture_rust_single_tool_call
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use liter_llm::BatchClient;
use liter_llm::FileClient;
use liter_llm::LlmClient;
use liter_llm::ResponseClient;

#[tokio::main]
async fn main() {
    let request_json: serde_json::Value = serde_json::from_str(r#"{"messages":[{"content":"What is the weather in San Francisco?","role":"user"}],"model":"gpt-4","tool_choice":"auto","tools":[{"function":{"description":"Get the current weather for a given location","name":"get_weather","parameters":{"properties":{"location":{"description":"The city and state, e.g. San Francisco, CA","type":"string"},"unit":{"description":"The temperature unit to use","enum":["celsius","fahrenheit"],"type":"string"}},"required":["location"],"type":"object"}},"type":"function"}]}"#).unwrap();
    let request = serde_json::from_value(request_json).unwrap();
    let client = liter_llm::create_client("test-key".to_string(), None, None, None, None).unwrap();
    let _ = client.chat(request).await;
}

```
