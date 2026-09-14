#![cfg(feature = "native-http")]
use liter_llm::client::{ClientConfigBuilder, DefaultClient, LlmClient};
use liter_llm::types::{EmbeddingInput, EmbeddingRequest};
use serde_json::{Value, json};
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::TcpListener,
    thread,
};

const MODEL: &str = "dashscope/tongyi-embedding-vision-plus";
fn request() -> EmbeddingRequest {
    EmbeddingRequest {
        model: MODEL.into(),
        input: EmbeddingInput::Multiple(vec!["first".into(), "second".into()]),
        encoding_format: None,
        dimensions: Some(1152),
        user: None,
    }
}
fn native(indices: &[usize], dimensions: usize) -> Value {
    json!({"output":{"embeddings":indices.iter().map(|index|json!({"index":index,"embedding":vec![0.1;dimensions]})).collect::<Vec<_>>()},"usage":{"input_tokens":12}})
}
fn server(responses: Vec<(u16, Value)>) -> (String, thread::JoinHandle<Vec<Value>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let handle = thread::spawn(move || {
        let mut bodies = Vec::new();
        for (status, body) in responses {
            let (mut socket, _) = listener.accept().unwrap();
            socket
                .set_read_timeout(Some(std::time::Duration::from_secs(10)))
                .unwrap();
            let mut reader = BufReader::new(socket.try_clone().unwrap());
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            assert!(line.starts_with("POST /services/embeddings/multimodal-embedding/multimodal-embedding "));
            let mut length = 0;
            let mut authorized = false;
            loop {
                line.clear();
                reader.read_line(&mut line).unwrap();
                if line.trim().is_empty() {
                    break;
                }
                if let Some((name, value)) = line.split_once(':') {
                    if name.eq_ignore_ascii_case("content-length") {
                        length = value.trim().parse().unwrap();
                    }
                    if name.eq_ignore_ascii_case("authorization") {
                        authorized = value.trim() == "Bearer fixture-key";
                    }
                }
            }
            assert!(authorized);
            let mut bytes = vec![0; length];
            reader.read_exact(&mut bytes).unwrap();
            bodies.push(serde_json::from_slice(&bytes).unwrap());
            let body = body.to_string();
            write!(socket,"HTTP/1.1 {status} Response\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len()).unwrap();
        }
        bodies
    });
    (url, handle)
}
fn client(url: &str, retries: u32) -> DefaultClient {
    DefaultClient::new(
        ClientConfigBuilder::new("fixture-key")
            .base_url(url)
            .max_retries(retries)
            .build(),
        Some(MODEL),
    )
    .unwrap()
}
#[tokio::test]
async fn dashscope_translates_and_orders_complete_batch() {
    let (url, server) = server(vec![(200, native(&[1, 0], 1152))]);
    let result = client(&url, 0).embed(request()).await.unwrap();
    assert_eq!(result.data.len(), 2);
    assert_eq!(result.data[0].index, 0);
    assert_eq!(result.data[1].index, 1);
    assert_eq!(result.model, "tongyi-embedding-vision-plus");
    assert_eq!(result.usage.unwrap().prompt_tokens, 12);
    assert_eq!(
        server.join().unwrap()[0],
        json!({"model":"tongyi-embedding-vision-plus","input":{"contents":[{"text":"first"},{"text":"second"}]}})
    );
}
#[tokio::test]
async fn dashscope_rejects_invalid_vectors_and_incomplete_batches() {
    for body in [
        native(&[0, 1], 3),
        native(&[0, 0], 1152),
        native(&[0], 1152),
        json!({"output":{"embeddings":[{"index":0,"embedding":[null]}]}}),
    ] {
        let (url, server) = server(vec![(200, body)]);
        assert!(client(&url, 0).embed(request()).await.is_err());
        server.join().unwrap();
    }
}
#[tokio::test]
async fn dashscope_provider_failure_and_recovery() {
    let (url, server) = server(vec![
        (503, json!({"code":"Unavailable","message":"temporary"})),
        (200, native(&[0, 1], 1152)),
    ]);
    assert_eq!(client(&url, 1).embed(request()).await.unwrap().data.len(), 2);
    assert_eq!(server.join().unwrap().len(), 2);
}
#[tokio::test]
async fn dashscope_rejects_wrong_requested_dimensions_before_network() {
    let mut req = request();
    req.dimensions = Some(1536);
    let result = client("http://127.0.0.1:1", 0).embed(req).await;
    assert!(matches!(
        result,
        Err(liter_llm::LiterLlmError::BadRequest { status: 400, .. })
    ));
}
