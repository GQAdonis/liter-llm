use serde::{Deserialize, Serialize};

/// Request to create a structured response.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateResponseRequest {
    /// Model ID.
    pub model: String,
    /// Input data to process (e.g., a document to extract from).
    pub input: serde_json::Value,
    /// Instructions for processing the input.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Available tools the model can use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ResponseTool>>,
    /// Sampling temperature in `[0.0, 2.0]`. Defaults to 1.0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Maximum output tokens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u64>,
    /// Optional metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// Provider-specific extra parameters merged into the request body.
    /// Use for provider extensions not modeled directly, such as OpenAI's
    /// Responses API `reasoning.effort` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_body: Option<serde_json::Value>,
    /// Whether to stream the response.
    ///
    /// Managed by the client layer — do not set directly.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

/// A tool available for the response request.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ResponseTool {
    /// Tool type (e.g., "extractor", "search").
    #[serde(rename = "type")]
    pub tool_type: String,
    /// Tool configuration (flattened into the object).
    #[serde(flatten)]
    pub config: serde_json::Value,
}

/// Response from a structured response request.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ResponseObject {
    /// Unique response ID.
    pub id: String,
    /// Object type (e.g., `"response"`).
    pub object: String,
    /// Unix timestamp of response creation.
    pub created_at: u64,
    /// Model used to generate the response.
    pub model: String,
    /// Status (e.g., `"succeeded"`, `"failed"`).
    pub status: String,
    /// Output items from the response.
    pub output: Vec<ResponseOutputItem>,
    /// Token usage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ResponseUsage>,
    /// Error details (if status is "failed").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
}

/// A single output item from the response.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ResponseOutputItem {
    /// Output type (e.g., `"text"`, `"object"`, `"error"`).
    #[serde(rename = "type")]
    pub item_type: String,
    /// Output content (flattened into the object).
    #[serde(flatten)]
    pub content: serde_json::Value,
}

/// Token usage for a response.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseUsage {
    /// Input tokens used.
    pub input_tokens: u64,
    /// Output tokens used.
    pub output_tokens: u64,
    /// Total tokens used.
    pub total_tokens: u64,
}

/// An incremental text delta for an output item's text content
/// (`response.output_text.delta`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ResponseOutputTextDelta {
    /// ID of the output item this delta belongs to.
    pub item_id: String,
    /// Index of the output item within `ResponseObject::output`.
    pub output_index: u32,
    /// Index of the content part within the output item.
    pub content_index: u32,
    /// The incremental text delta.
    pub delta: String,
}

/// An incremental function-call arguments delta
/// (`response.function_call_arguments.delta`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ResponseFunctionCallArgumentsDelta {
    /// ID of the function-call output item this delta belongs to.
    pub item_id: String,
    /// Index of the output item within `ResponseObject::output`.
    pub output_index: u32,
    /// The incremental arguments JSON-fragment delta.
    pub delta: String,
}

/// An output item lifecycle event (`response.output_item.added` /
/// `response.output_item.done`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ResponseOutputItemEvent {
    /// Index of the output item within `ResponseObject::output`.
    pub output_index: u32,
    /// The output item that was added or completed.
    pub item: ResponseOutputItem,
}

/// A web search tool-call lifecycle event (`response.web_search_call.in_progress` /
/// `.searching` / `.completed`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ResponseWebSearchCallEvent {
    /// Index of the output item within `ResponseObject::output`.
    pub output_index: u32,
    /// ID of the web search call output item.
    pub item_id: String,
}

/// A terminal response event carrying the final response object
/// (`response.completed` / `response.incomplete` / `response.failed`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ResponseCompletedEvent {
    /// The final response object.
    pub response: ResponseObject,
}

/// An SSE event whose `type` was not recognized by this version of the
/// client.
///
/// Carried so the stream survives new event types the API adds after this
/// client was released, rather than failing the whole stream.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnknownResponseStreamEvent {
    /// The raw, unrecognized `type` value from the SSE payload.
    pub event_type: String,
    /// The complete raw JSON payload (including `type`), for forward-compatible inspection.
    pub raw: serde_json::Value,
}

/// A single Server-Sent Event from the Responses API streaming endpoint.
///
/// Each variant corresponds to a `response.*` SSE event `type`. Unknown or
/// future event types deserialize into [`ResponseStreamEvent::Unknown`]
/// rather than failing deserialization, so a stream is never broken by a new
/// event type introduced by the API after this client was released.
///
/// `Completed`, `Incomplete`, and `Failed` are the terminal events in the
/// Responses streaming model.
#[derive(Debug, Clone, PartialEq)]
pub enum ResponseStreamEvent {
    /// Incremental text delta for an output item's text content
    /// (`response.output_text.delta`).
    OutputTextDelta(ResponseOutputTextDelta),
    /// Incremental function-call arguments delta
    /// (`response.function_call_arguments.delta`).
    FunctionCallArgumentsDelta(ResponseFunctionCallArgumentsDelta),
    /// A new output item was added to the response (`response.output_item.added`).
    OutputItemAdded(ResponseOutputItemEvent),
    /// An output item finished streaming (`response.output_item.done`).
    OutputItemDone(ResponseOutputItemEvent),
    /// A web search call started (`response.web_search_call.in_progress`).
    WebSearchCallInProgress(ResponseWebSearchCallEvent),
    /// A web search call is actively searching (`response.web_search_call.searching`).
    WebSearchCallSearching(ResponseWebSearchCallEvent),
    /// A web search call finished (`response.web_search_call.completed`).
    WebSearchCallCompleted(ResponseWebSearchCallEvent),
    /// The response finished successfully (`response.completed`). Terminal.
    Completed(ResponseCompletedEvent),
    /// The response finished before completion, e.g. due to a length or
    /// content-filter limit (`response.incomplete`). Terminal.
    Incomplete(ResponseCompletedEvent),
    /// The response failed (`response.failed`). Terminal.
    Failed(ResponseCompletedEvent),
    /// An event type not recognized by this version of the client.
    Unknown(UnknownResponseStreamEvent),
}

impl Serialize for ResponseStreamEvent {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        #[serde(tag = "type")]
        enum Tagged<'a> {
            #[serde(rename = "response.output_text.delta")]
            OutputTextDelta(&'a ResponseOutputTextDelta),
            #[serde(rename = "response.function_call_arguments.delta")]
            FunctionCallArgumentsDelta(&'a ResponseFunctionCallArgumentsDelta),
            #[serde(rename = "response.output_item.added")]
            OutputItemAdded(&'a ResponseOutputItemEvent),
            #[serde(rename = "response.output_item.done")]
            OutputItemDone(&'a ResponseOutputItemEvent),
            #[serde(rename = "response.web_search_call.in_progress")]
            WebSearchCallInProgress(&'a ResponseWebSearchCallEvent),
            #[serde(rename = "response.web_search_call.searching")]
            WebSearchCallSearching(&'a ResponseWebSearchCallEvent),
            #[serde(rename = "response.web_search_call.completed")]
            WebSearchCallCompleted(&'a ResponseWebSearchCallEvent),
            #[serde(rename = "response.completed")]
            Completed(&'a ResponseCompletedEvent),
            #[serde(rename = "response.incomplete")]
            Incomplete(&'a ResponseCompletedEvent),
            #[serde(rename = "response.failed")]
            Failed(&'a ResponseCompletedEvent),
        }

        match self {
            Self::OutputTextDelta(event) => Tagged::OutputTextDelta(event).serialize(serializer),
            Self::FunctionCallArgumentsDelta(event) => Tagged::FunctionCallArgumentsDelta(event).serialize(serializer),
            Self::OutputItemAdded(event) => Tagged::OutputItemAdded(event).serialize(serializer),
            Self::OutputItemDone(event) => Tagged::OutputItemDone(event).serialize(serializer),
            Self::WebSearchCallInProgress(event) => Tagged::WebSearchCallInProgress(event).serialize(serializer),
            Self::WebSearchCallSearching(event) => Tagged::WebSearchCallSearching(event).serialize(serializer),
            Self::WebSearchCallCompleted(event) => Tagged::WebSearchCallCompleted(event).serialize(serializer),
            Self::Completed(event) => Tagged::Completed(event).serialize(serializer),
            Self::Incomplete(event) => Tagged::Incomplete(event).serialize(serializer),
            Self::Failed(event) => Tagged::Failed(event).serialize(serializer),
            Self::Unknown(event) => event.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for ResponseStreamEvent {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let event_type = value
            .get("type")
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default()
            .to_owned();

        let variant = match event_type.as_str() {
            "response.output_text.delta" => serde_json::from_value(value.clone()).map(Self::OutputTextDelta),
            "response.function_call_arguments.delta" => {
                serde_json::from_value(value.clone()).map(Self::FunctionCallArgumentsDelta)
            }
            "response.output_item.added" => serde_json::from_value(value.clone()).map(Self::OutputItemAdded),
            "response.output_item.done" => serde_json::from_value(value.clone()).map(Self::OutputItemDone),
            "response.web_search_call.in_progress" => {
                serde_json::from_value(value.clone()).map(Self::WebSearchCallInProgress)
            }
            "response.web_search_call.searching" => {
                serde_json::from_value(value.clone()).map(Self::WebSearchCallSearching)
            }
            "response.web_search_call.completed" => {
                serde_json::from_value(value.clone()).map(Self::WebSearchCallCompleted)
            }
            "response.completed" => serde_json::from_value(value.clone()).map(Self::Completed),
            "response.incomplete" => serde_json::from_value(value.clone()).map(Self::Incomplete),
            "response.failed" => serde_json::from_value(value.clone()).map(Self::Failed),
            _ => return Ok(Self::Unknown(UnknownResponseStreamEvent { event_type, raw: value })),
        };

        variant.map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod stream_event_tests {
    use super::*;

    #[test]
    fn output_text_delta_extracts_exact_delta_value() {
        let payload = serde_json::json!({
            "type": "response.output_text.delta",
            "item_id": "msg_1",
            "output_index": 0,
            "content_index": 0,
            "delta": "Hello, world",
        });
        let event: ResponseStreamEvent = serde_json::from_value(payload).expect("should deserialize");
        match event {
            ResponseStreamEvent::OutputTextDelta(delta) => {
                assert_eq!(delta.delta, "Hello, world");
                assert_eq!(delta.item_id, "msg_1");
                assert_eq!(delta.output_index, 0);
                assert_eq!(delta.content_index, 0);
            }
            other => panic!("expected OutputTextDelta, got {other:?}"),
        }
    }

    #[test]
    fn function_call_arguments_delta_extracts_exact_delta_value() {
        let payload = serde_json::json!({
            "type": "response.function_call_arguments.delta",
            "item_id": "fc_1",
            "output_index": 2,
            "delta": "{\"city\":",
        });
        let event: ResponseStreamEvent = serde_json::from_value(payload).expect("should deserialize");
        match event {
            ResponseStreamEvent::FunctionCallArgumentsDelta(delta) => {
                assert_eq!(delta.delta, "{\"city\":");
                assert_eq!(delta.item_id, "fc_1");
                assert_eq!(delta.output_index, 2);
            }
            other => panic!("expected FunctionCallArgumentsDelta, got {other:?}"),
        }
    }

    #[test]
    fn output_item_added_carries_the_output_item() {
        let payload = serde_json::json!({
            "type": "response.output_item.added",
            "output_index": 0,
            "item": { "type": "message", "id": "msg_1" },
        });
        let event: ResponseStreamEvent = serde_json::from_value(payload).expect("should deserialize");
        match event {
            ResponseStreamEvent::OutputItemAdded(added) => {
                assert_eq!(added.output_index, 0);
                assert_eq!(added.item.item_type, "message");
                assert_eq!(added.item.content.get("id").and_then(|v| v.as_str()), Some("msg_1"));
            }
            other => panic!("expected OutputItemAdded, got {other:?}"),
        }
    }

    #[test]
    fn output_item_done_carries_the_output_item() {
        let payload = serde_json::json!({
            "type": "response.output_item.done",
            "output_index": 1,
            "item": { "type": "message", "id": "msg_2" },
        });
        let event: ResponseStreamEvent = serde_json::from_value(payload).expect("should deserialize");
        match event {
            ResponseStreamEvent::OutputItemDone(done) => {
                assert_eq!(done.output_index, 1);
                assert_eq!(done.item.item_type, "message");
                assert_eq!(done.item.content.get("id").and_then(|v| v.as_str()), Some("msg_2"));
            }
            other => panic!("expected OutputItemDone, got {other:?}"),
        }
    }

    #[test]
    fn web_search_call_in_progress_carries_item_id() {
        let payload = serde_json::json!({
            "type": "response.web_search_call.in_progress",
            "output_index": 0,
            "item_id": "ws_1",
        });
        let event: ResponseStreamEvent = serde_json::from_value(payload).expect("should deserialize");
        match event {
            ResponseStreamEvent::WebSearchCallInProgress(call) => {
                assert_eq!(call.item_id, "ws_1");
                assert_eq!(call.output_index, 0);
            }
            other => panic!("expected WebSearchCallInProgress, got {other:?}"),
        }
    }

    #[test]
    fn web_search_call_searching_carries_item_id() {
        let payload = serde_json::json!({
            "type": "response.web_search_call.searching",
            "output_index": 0,
            "item_id": "ws_1",
        });
        let event: ResponseStreamEvent = serde_json::from_value(payload).expect("should deserialize");
        match event {
            ResponseStreamEvent::WebSearchCallSearching(call) => {
                assert_eq!(call.item_id, "ws_1");
                assert_eq!(call.output_index, 0);
            }
            other => panic!("expected WebSearchCallSearching, got {other:?}"),
        }
    }

    #[test]
    fn web_search_call_completed_carries_item_id() {
        let payload = serde_json::json!({
            "type": "response.web_search_call.completed",
            "output_index": 0,
            "item_id": "ws_1",
        });
        let event: ResponseStreamEvent = serde_json::from_value(payload).expect("should deserialize");
        match event {
            ResponseStreamEvent::WebSearchCallCompleted(call) => {
                assert_eq!(call.item_id, "ws_1");
                assert_eq!(call.output_index, 0);
            }
            other => panic!("expected WebSearchCallCompleted, got {other:?}"),
        }
    }

    #[test]
    fn completed_carries_the_final_response_object() {
        let payload = serde_json::json!({
            "type": "response.completed",
            "response": {
                "id": "resp_1",
                "object": "response",
                "created_at": 1_700_000_000,
                "model": "gpt-4o",
                "status": "completed",
                "output": [],
            },
        });
        let event: ResponseStreamEvent = serde_json::from_value(payload).expect("should deserialize");
        match event {
            ResponseStreamEvent::Completed(completed) => {
                assert_eq!(completed.response.id, "resp_1");
                assert_eq!(completed.response.status, "completed");
            }
            other => panic!("expected Completed, got {other:?}"),
        }
    }

    #[test]
    fn failed_carries_the_final_response_object() {
        let payload = serde_json::json!({
            "type": "response.failed",
            "response": {
                "id": "resp_2",
                "object": "response",
                "created_at": 1_700_000_000,
                "model": "gpt-4o",
                "status": "failed",
                "output": [],
                "error": { "message": "rate limited" },
            },
        });
        let event: ResponseStreamEvent = serde_json::from_value(payload).expect("should deserialize");
        match event {
            ResponseStreamEvent::Failed(failed) => {
                assert_eq!(failed.response.id, "resp_2");
                assert_eq!(failed.response.status, "failed");
                assert_eq!(
                    failed
                        .response
                        .error
                        .as_ref()
                        .and_then(|e| e.get("message"))
                        .and_then(|v| v.as_str()),
                    Some("rate limited")
                );
            }
            other => panic!("expected Failed, got {other:?}"),
        }
    }

    #[test]
    fn unknown_event_type_deserializes_to_catch_all_variant() {
        let payload = serde_json::json!({
            "type": "response.some_future_event",
            "foo": "bar",
        });
        let event: ResponseStreamEvent = serde_json::from_value(payload).expect("should deserialize");
        match event {
            ResponseStreamEvent::Unknown(unknown) => {
                assert_eq!(unknown.event_type, "response.some_future_event");
                assert_eq!(unknown.raw.get("foo").and_then(|v| v.as_str()), Some("bar"));
            }
            other => panic!("expected Unknown, got {other:?}"),
        }
    }
}
