---
id: fixture_rust_stream_with_tool_calls
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
    let request_json: serde_json::Value = serde_json::from_str(r#"{"messages":[{"content":"What is the weather in NYC?","role":"user"}],"model":"gpt-4","stream":true,"tools":[{"function":{"description":"Get the current weather for a given location","name":"get_weather","parameters":{"properties":{"location":{"description":"The city and state, e.g. New York, NY","type":"string"}},"required":["location"],"type":"object"}},"type":"function"}]}"#).unwrap();
    let request = serde_json::from_value(request_json).unwrap();
    let client = liter_llm::create_client("test-key".to_string(), None, None, None, None).unwrap();
    let stream = client.chat_stream(request).await.expect("call failed");
    let chunks: Vec<_> = tokio_stream::StreamExt::collect::<Vec<_>>(stream).await
        .into_iter()
        .map(|r| r.expect("stream item failed"))
        .collect();
}

```
