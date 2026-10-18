#![cfg(feature = "native-http")]

mod common;

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::thread;

use futures_util::StreamExt;
use liter_llm::client::{ClientConfigBuilder, DefaultClient, LlmClient, LlmClientRaw};
use liter_llm::types::{ChatCompletionRequest, ChatCompletionResponse, Message, Usage, UserMessage};
use serde_json::{Value, json};

const REQUEST_MODEL: &str = "gemini-2.5-flash";
const REPORTED_MODEL: &str = "gemini-2.5-flash-lite";

fn payload(model_version: Value) -> Value {
    json!({
        "modelVersion": model_version,
        "candidates": [{"content": {"parts": [{"text": "Hello"}]}, "finishReason": "STOP"}],
        "usageMetadata": {"promptTokenCount": 337, "candidatesTokenCount": 608, "totalTokenCount": 945}
    })
}

fn request(prefix: &str) -> ChatCompletionRequest {
    ChatCompletionRequest {
        model: format!("{prefix}/{REQUEST_MODEL}"),
        messages: vec![Message::User(UserMessage {
            content: "Hello".into(),
            ..Default::default()
        })],
        ..Default::default()
    }
}

fn client(prefix: &str, body: String, content_type: &'static str) -> (DefaultClient, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind response server");
    let base_url = format!("http://{}", listener.local_addr().expect("server address"));
    let handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept request");
        let mut reader = BufReader::new(stream.try_clone().expect("clone stream"));
        let mut content_length = 0;
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).expect("read request headers");
            if line.trim().is_empty() {
                break;
            }
            if let Some((name, value)) = line.split_once(':')
                && name.eq_ignore_ascii_case("content-length")
            {
                content_length = value.trim().parse().expect("request content length");
            }
        }
        reader
            .read_exact(&mut vec![0; content_length])
            .expect("read request body");
        write!(
            stream,
            "HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
        .expect("write response");
    });
    let config = ClientConfigBuilder::new("test-key").base_url(base_url).build();
    let client = DefaultClient::new(config, Some(&format!("{prefix}/{REQUEST_MODEL}"))).expect("create client");
    (client, handle)
}

fn assert_usage_and_cost(model: &str, usage: Option<&Usage>, expected_model: &str) {
    assert_eq!(model, expected_model);
    let usage = usage.expect("normalized usage");
    assert_eq!(
        (usage.prompt_tokens, usage.completion_tokens, usage.total_tokens),
        (337, 608, 945)
    );
    let cost = liter_llm::cost::completion_cost(model, usage.prompt_tokens, usage.completion_tokens)
        .expect("model attribution permits pricing");
    let expected = liter_llm::cost::completion_cost(expected_model, 337, 608).expect("fixture model pricing");
    assert_eq!(cost, expected);
}

fn assert_response(response: &ChatCompletionResponse, expected_model: &str) {
    assert_usage_and_cost(&response.model, response.usage.as_ref(), expected_model);
    assert_eq!(
        response.estimated_cost(),
        liter_llm::cost::completion_cost(expected_model, 337, 608)
    );
}

#[tokio::test]
async fn should_preserve_reported_or_requested_gemini_model_in_standard_chat() {
    for prefix in ["gemini", "google_ai", "vertex_ai"] {
        for metadata in [json!(REPORTED_MODEL), Value::Null, json!(""), json!(42)] {
            let expected = metadata
                .as_str()
                .filter(|value| !value.is_empty())
                .map_or_else(|| format!("{prefix}/{REQUEST_MODEL}"), str::to_owned);
            let (client, server) = client(prefix, payload(metadata).to_string(), "application/json");
            let response = client.chat(request(prefix)).await.expect("chat succeeds");
            assert_response(&response, &expected);
            server.join().expect("response server completed");
        }
    }
}

#[tokio::test]
async fn should_preserve_model_attribution_without_altering_raw_gemini_response() {
    for prefix in ["gemini", "vertex_ai"] {
        for metadata in [json!(REPORTED_MODEL), Value::Null] {
            let expected = metadata
                .as_str()
                .map_or_else(|| format!("{prefix}/{REQUEST_MODEL}"), str::to_owned);
            let raw_body = payload(metadata);
            let (client, server) = client(prefix, raw_body.to_string(), "application/json");
            let response = client.chat_raw(request(prefix)).await.expect("raw chat succeeds");
            assert_response(&response.data, &expected);
            assert_eq!(response.raw_response, Some(raw_body));
            server.join().expect("response server completed");
        }
    }
}

async fn assert_stream_model(raw: bool, metadata: Value) {
    for prefix in ["gemini", "vertex_ai"] {
        let expected = metadata
            .as_str()
            .map_or_else(|| format!("{prefix}/{REQUEST_MODEL}"), str::to_owned);
        let first = payload(metadata.clone());
        let mut last = payload(Value::Null);
        last.as_object_mut().expect("fixture object").remove("modelVersion");
        let body = format!("data: {first}\n\ndata: {last}\n\n");
        let (client, server) = client(prefix, body, "text/event-stream");
        let mut stream = if raw {
            client
                .chat_stream_raw(request(prefix))
                .await
                .expect("raw stream succeeds")
                .stream
        } else {
            client.chat_stream(request(prefix)).await.expect("stream succeeds")
        };
        let mut count = 0;
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.expect("valid chunk");
            assert_usage_and_cost(&chunk.model, chunk.usage.as_ref(), &expected);
            count += 1;
        }
        assert_eq!(count, 2);
        server.join().expect("response server completed");
    }
}

#[tokio::test]
async fn should_preserve_reported_or_requested_model_through_gemini_stream() {
    assert_stream_model(false, json!(REPORTED_MODEL)).await;
    assert_stream_model(false, Value::Null).await;
}

#[tokio::test]
async fn should_preserve_reported_or_requested_model_through_raw_gemini_stream() {
    assert_stream_model(true, json!(REPORTED_MODEL)).await;
    assert_stream_model(true, Value::Null).await;
}
