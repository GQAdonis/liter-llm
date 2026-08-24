use std::io::{Read, Write};
use std::net::TcpListener;

use liter_llm::client::{ClientConfigBuilder, DefaultClient, LlmClient};
use liter_llm::provider::{OutboundPolicy, set_outbound_policy};
use liter_llm::{ChatCompletionRequest, LiterLlmError};
use serial_test::serial;

fn chat_request() -> ChatCompletionRequest {
    serde_json::from_value(serde_json::json!({
        "model": "test-model",
        "messages": [{"role": "user", "content": "Hi"}]
    }))
    .expect("chat request")
}

fn one_shot_chat_server() -> (String, std::thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind chat server");
    let address = listener.local_addr().expect("chat server address");
    let handle = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept chat request");
        let mut request = [0_u8; 4096];
        let _ = stream.read(&mut request).expect("read chat request");
        let response_body = serde_json::json!({
            "id": "chatcmpl-test",
            "object": "chat.completion",
            "created": 1_700_000_000_u64,
            "model": "test-model",
            "choices": [{
                "index": 0,
                "message": {"role": "assistant", "content": "ok"},
                "finish_reason": "stop"
            }]
        })
        .to_string();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            response_body.len(),
            response_body
        );
        stream.write_all(response.as_bytes()).expect("write chat response");
    });
    (format!("http://{address}"), handle)
}

#[tokio::test]
#[serial(outbound_policy)]
async fn client_base_url_rejects_private_target_under_deny_private() {
    set_outbound_policy(OutboundPolicy::DenyPrivate);
    let config = ClientConfigBuilder::new("test-key")
        .base_url("http://127.0.0.1:9")
        .max_retries(0)
        .build();
    let client = DefaultClient::new(config, None).expect("client creation");

    let result = client.chat(chat_request()).await;
    set_outbound_policy(OutboundPolicy::Off);

    assert!(
        matches!(result, Err(LiterLlmError::OutboundForbidden { .. })),
        "request-controlled private base_url must be rejected: {result:?}"
    );
}

#[tokio::test]
#[serial(outbound_policy)]
async fn client_base_url_allows_local_mock_when_policy_is_off() {
    set_outbound_policy(OutboundPolicy::Off);
    let (base_url, server) = one_shot_chat_server();
    let config = ClientConfigBuilder::new("test-key")
        .base_url(base_url)
        .max_retries(0)
        .build();
    let client = DefaultClient::new(config, None).expect("client creation");

    let result = client.chat(chat_request()).await;

    server.join().expect("chat server thread");
    assert!(result.is_ok(), "Off must preserve embedded local endpoints: {result:?}");
}
