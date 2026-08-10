---
id: fixture_rust_tool_choice_required
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
    let request_json: serde_json::Value = serde_json::from_str(r#"{"messages":[{"content":"What is the weather today?","role":"user"}],"model":"gpt-4","tool_choice":"required","tools":[{"function":{"description":"Get the current weather for a given location","name":"get_weather","parameters":{"properties":{"location":{"description":"The city name","type":"string"}},"required":["location"],"type":"object"}},"type":"function"}]}"#).unwrap();
    let request = serde_json::from_value(request_json).unwrap();
    let client = liter_llm::create_client("test-key".to_string(), None, None, None, None).unwrap();
    let _ = client.chat(request).await;
}

```
