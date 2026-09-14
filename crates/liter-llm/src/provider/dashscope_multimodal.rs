//! Native DashScope independent embeddings behind the OpenAI embedding contract.
use std::borrow::Cow;

use serde::Deserialize;
use serde_json::{Value, json};

use super::Provider;
use crate::error::{LiterLlmError, Result};

const MODEL: &str = "tongyi-embedding-vision-plus";
const DIMENSIONS: usize = 1152;

pub(crate) struct DashScopeMultimodalProvider;

fn invalid(message: &str) -> LiterLlmError {
    LiterLlmError::BadRequest {
        message: message.into(),
        status: 400,
    }
}
fn malformed() -> LiterLlmError {
    LiterLlmError::ServerError {
        message: "DashScope returned invalid, incomplete, or non-finite embeddings".into(),
        status: 502,
    }
}

impl Provider for DashScopeMultimodalProvider {
    fn name(&self) -> &str {
        "dashscope"
    }
    fn base_url(&self) -> &str {
        "https://dashscope-intl.aliyuncs.com/api/v1"
    }
    fn env_var(&self) -> Option<&str> {
        Some("DASHSCOPE_API_KEY")
    }
    fn auth_header<'a>(&'a self, key: &'a str) -> Option<(Cow<'static, str>, Cow<'a, str>)> {
        Some((Cow::Borrowed("Authorization"), Cow::Owned(format!("Bearer {key}"))))
    }
    fn matches_model(&self, model: &str) -> bool {
        self.strip_model_prefix(model) == MODEL
    }
    fn embeddings_path(&self) -> &str {
        "/services/embeddings/multimodal-embedding/multimodal-embedding"
    }
    fn transform_request(&self, body: &mut Value) -> Result<()> {
        if body.get("model").and_then(Value::as_str) != Some(MODEL) {
            return Err(invalid("unsupported DashScope multimodal embedding model"));
        }
        if body
            .get("dimensions")
            .is_some_and(|v| v.as_u64() != Some(DIMENSIONS as u64))
        {
            return Err(invalid("tongyi-embedding-vision-plus requires 1152 dimensions"));
        }
        if body.get("encoding_format").is_some_and(|v| v.as_str() != Some("float")) {
            return Err(invalid("DashScope multimodal embeddings require float encoding"));
        }
        let contents = match body.get("input") {
            Some(Value::String(text)) if !text.trim().is_empty() => vec![json!({"text":text})],
            Some(Value::Array(texts)) if !texts.is_empty() => texts
                .iter()
                .map(|text| {
                    text.as_str()
                        .filter(|text| !text.trim().is_empty())
                        .map(|text| json!({"text":text}))
                        .ok_or_else(|| {
                            invalid("input must contain nonempty text strings; fused inputs are unsupported")
                        })
                })
                .collect::<Result<Vec<_>>>()?,
            _ => return Err(invalid("input must be a nonempty text string or batch of text strings")),
        };
        // No dimensions parameter: this model's native API uses fixed dimensions.
        *body = json!({"model":MODEL,"input":{"contents":contents}});
        Ok(())
    }
    fn transform_response(&self, body: &mut Value) -> Result<()> {
        let mut response: NativeResponse = serde_json::from_value(body.take()).map_err(|_| malformed())?;
        response.output.embeddings.sort_by_key(|item| item.index);
        if response.output.embeddings.is_empty()
            || response.output.embeddings.iter().enumerate().any(|(i, item)| {
                item.index != i || item.embedding.len() != DIMENSIONS || item.embedding.iter().any(|v| !v.is_finite())
            })
        {
            return Err(malformed());
        }
        let data: Vec<_> = response
            .output
            .embeddings
            .into_iter()
            .map(|item| json!({"object":"embedding","index":item.index,"embedding":item.embedding}))
            .collect();
        *body = json!({"object":"list","model":MODEL,"data":data,
            "usage":{"prompt_tokens":response.usage.input_tokens,"completion_tokens":0,
                     "total_tokens":response.usage.input_tokens}});
        Ok(())
    }
}

#[derive(Deserialize)]
struct NativeResponse {
    output: NativeOutput,
    #[serde(default)]
    usage: NativeUsage,
}
#[derive(Deserialize)]
struct NativeOutput {
    embeddings: Vec<NativeEmbedding>,
}
#[derive(Deserialize)]
struct NativeEmbedding {
    index: usize,
    embedding: Vec<f32>,
}
#[derive(Default, Deserialize)]
struct NativeUsage {
    #[serde(default)]
    input_tokens: u64,
}
