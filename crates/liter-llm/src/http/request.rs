use std::future::Future;

use bytes::Bytes;

use crate::error::{LiterLlmError, Result};
use crate::http::retry;

/// Extract an optional `Retry-After` delay from a response.
pub(crate) fn retry_after_from_response(resp: &reqwest::Response) -> Option<std::time::Duration> {
    let value = resp.headers().get(reqwest::header::RETRY_AFTER)?.to_str().ok()?;
    retry::parse_retry_after(value)
}

/// Sleep for a retry back-off delay, on native or WASM targets.
async fn sleep_for_retry(delay: std::time::Duration) {
    #[cfg(not(target_arch = "wasm32"))]
    tokio::time::sleep(delay).await;
    #[cfg(target_arch = "wasm32")]
    gloo_timers::future::sleep(std::time::Duration::from_millis(delay.as_millis() as u64)).await;
}

/// Drive a single-request closure through the retry / back-off loop.
///
/// `send` is called once per attempt and must return a future that resolves to
/// a raw `reqwest::Response` (or a transport-level error).  The helper handles:
///
/// - Attempt counting and the `max_retries` budget.
/// - Parsing the `Retry-After` header before consuming the response body.
/// - Exponential back-off via `retry::should_retry`, including for
///   transport-level failures (connection reset, DNS failure, timeout) that
///   never produce a response at all.
/// - Reading the error body and mapping it to `LiterLlmError` on final failure.
///
/// On success the **successful** `Response` is returned so the caller can
/// choose how to consume the body (JSON deserialisation, byte stream, …).
pub(crate) async fn with_retry<F, Fut>(max_retries: u32, mut send: F) -> Result<reqwest::Response>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = std::result::Result<reqwest::Response, reqwest::Error>>,
{
    let mut attempt = 0u32;

    loop {
        let resp = match send().await {
            Ok(resp) => resp,
            Err(transport_error) => {
                // ~keep A transport-level error means no response was ever received, so it
                // must go through the same retry budget as a 5xx — otherwise a single
                // connection reset fails the whole request even though `max_retries > 0`.
                if let Some(delay) = retry::should_retry_transport_error(attempt, max_retries) {
                    attempt += 1;
                    tracing::warn!(
                        error = %transport_error,
                        attempt,
                        max_retries,
                        "transport-level error sending request; retrying"
                    );
                    sleep_for_retry(delay).await;
                    continue;
                }
                return Err(LiterLlmError::from(transport_error));
            }
        };
        let status = resp.status().as_u16();

        if resp.status().is_success() {
            return Ok(resp);
        }

        let server_retry_after = retry_after_from_response(&resp);

        if let Some(delay) = retry::should_retry(status, attempt, max_retries, server_retry_after) {
            attempt += 1;
            sleep_for_retry(delay).await;
            continue;
        }

        let text = resp
            .text()
            .await
            .unwrap_or_else(|e| format!("(failed to read body: {e})"));
        return Err(LiterLlmError::from_status(status, &text, server_retry_after));
    }
}

/// Send a POST request with a JSON body and return the raw response JSON.
///
/// Like `post_json` but returns a `serde_json::Value` instead of deserializing
/// into a typed `T`.  This allows the caller to mutate the response (e.g. via a
/// provider `transform_response`) before deserializing into the canonical type.
///
/// Retries on 429 / 5xx according to `max_retries`.
#[tracing::instrument(
    skip_all,
    fields(
        http.method = "POST",
        http.url = %url,
        http.status_code = tracing::field::Empty,
        http.retry_count = tracing::field::Empty,
    )
)]
pub async fn post_json_raw(
    client: &reqwest::Client,
    url: &str,
    auth_header: Option<(&str, &str)>,
    extra_headers: &[(&str, &str)],
    body: Bytes,
    max_retries: u32,
) -> Result<serde_json::Value> {
    let mut retry_count = 0u32;

    let resp = with_retry(max_retries, || {
        let mut builder = client
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body.clone());
        if let Some((name, value)) = auth_header {
            builder = builder.header(name, value);
        }
        for (name, value) in extra_headers {
            builder = builder.header(*name, *value);
        }
        retry_count += 1;
        builder.send()
    })
    .await?;

    {
        let span = tracing::Span::current();
        span.record("http.status_code", resp.status().as_u16());
        span.record("http.retry_count", retry_count.saturating_sub(1));
    }

    resp.json::<serde_json::Value>().await.map_err(LiterLlmError::from)
}

/// Send a POST request with a JSON body and return the raw response bytes.
///
/// Identical to `post_json_raw` except it returns `bytes::Bytes` instead of
/// deserializing JSON.  Useful for endpoints that return binary data (e.g.
/// text-to-speech audio).
///
/// Retries on 429 / 5xx according to `max_retries`.
#[tracing::instrument(
    skip_all,
    fields(
        http.method = "POST",
        http.url = %url,
        http.status_code = tracing::field::Empty,
        http.retry_count = tracing::field::Empty,
    )
)]
pub async fn post_binary(
    client: &reqwest::Client,
    url: &str,
    auth_header: Option<(&str, &str)>,
    extra_headers: &[(&str, &str)],
    body: Bytes,
    max_retries: u32,
) -> Result<Bytes> {
    let mut retry_count = 0u32;

    let resp = with_retry(max_retries, || {
        let mut builder = client
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body.clone());
        if let Some((name, value)) = auth_header {
            builder = builder.header(name, value);
        }
        for (name, value) in extra_headers {
            builder = builder.header(*name, *value);
        }
        retry_count += 1;
        builder.send()
    })
    .await?;

    {
        let span = tracing::Span::current();
        span.record("http.status_code", resp.status().as_u16());
        span.record("http.retry_count", retry_count.saturating_sub(1));
    }

    resp.bytes().await.map_err(LiterLlmError::from)
}

/// Send a POST request with a multipart form body and return the raw response JSON.
///
/// Used for file uploads (Files API, audio transcription).  Multipart forms are
/// consumed by `send()` and cannot be cheaply cloned, so this function does
/// **not** retry on failure — file uploads are not idempotent anyway.
///
/// `auth_header` is `Some((name, value))` when the provider requires
/// authentication, or `None` when no auth header should be added.
#[tracing::instrument(
    skip_all,
    fields(
        http.method = "POST",
        http.url = %url,
        http.status_code = tracing::field::Empty,
    )
)]
pub async fn post_multipart(
    client: &reqwest::Client,
    url: &str,
    auth_header: Option<(&str, &str)>,
    extra_headers: &[(&str, &str)],
    form: reqwest::multipart::Form,
) -> Result<serde_json::Value> {
    let mut builder = client.post(url).multipart(form);
    if let Some((name, value)) = auth_header {
        builder = builder.header(name, value);
    }
    for (name, value) in extra_headers {
        builder = builder.header(*name, *value);
    }

    let resp = builder.send().await?;

    {
        let span = tracing::Span::current();
        span.record("http.status_code", resp.status().as_u16());
    }

    let status = resp.status().as_u16();
    if !resp.status().is_success() {
        let server_retry_after = retry_after_from_response(&resp);
        let text = resp
            .text()
            .await
            .unwrap_or_else(|e| format!("(failed to read body: {e})"));
        return Err(LiterLlmError::from_status(status, &text, server_retry_after));
    }

    resp.json::<serde_json::Value>().await.map_err(LiterLlmError::from)
}

/// Send a GET request and return the raw response JSON as `serde_json::Value`.
///
/// Returns a raw `serde_json::Value` without deserializing into a typed `T`.
/// Useful for endpoints where the caller needs to inspect or transform the
/// response before deserialization (e.g. GET /files/{id}, GET /batches/{id}).
///
/// Retries on 429 / 5xx according to `max_retries`.
#[tracing::instrument(
    skip_all,
    fields(
        http.method = "GET",
        http.url = %url,
        http.status_code = tracing::field::Empty,
        http.retry_count = tracing::field::Empty,
    )
)]
pub async fn get_json_raw(
    client: &reqwest::Client,
    url: &str,
    auth_header: Option<(&str, &str)>,
    extra_headers: &[(&str, &str)],
    max_retries: u32,
) -> Result<serde_json::Value> {
    let mut retry_count = 0u32;

    let resp = with_retry(max_retries, || {
        let mut builder = client.get(url);
        if let Some((name, value)) = auth_header {
            builder = builder.header(name, value);
        }
        for (name, value) in extra_headers {
            builder = builder.header(*name, *value);
        }
        retry_count += 1;
        builder.send()
    })
    .await?;

    {
        let span = tracing::Span::current();
        span.record("http.status_code", resp.status().as_u16());
        span.record("http.retry_count", retry_count.saturating_sub(1));
    }

    resp.json::<serde_json::Value>().await.map_err(LiterLlmError::from)
}

/// Send a DELETE request and return the raw response JSON.
///
/// Same retry/auth/header pattern as `get_json_raw` but uses the HTTP DELETE method.
/// Used for resource deletion endpoints (e.g. DELETE /files/{id}).
///
/// Retries on 429 / 5xx according to `max_retries`.
#[tracing::instrument(
    skip_all,
    fields(
        http.method = "DELETE",
        http.url = %url,
        http.status_code = tracing::field::Empty,
        http.retry_count = tracing::field::Empty,
    )
)]
pub async fn delete_json(
    client: &reqwest::Client,
    url: &str,
    auth_header: Option<(&str, &str)>,
    extra_headers: &[(&str, &str)],
    max_retries: u32,
) -> Result<serde_json::Value> {
    let mut retry_count = 0u32;

    let resp = with_retry(max_retries, || {
        let mut builder = client.delete(url);
        if let Some((name, value)) = auth_header {
            builder = builder.header(name, value);
        }
        for (name, value) in extra_headers {
            builder = builder.header(*name, *value);
        }
        retry_count += 1;
        builder.send()
    })
    .await?;

    {
        let span = tracing::Span::current();
        span.record("http.status_code", resp.status().as_u16());
        span.record("http.retry_count", retry_count.saturating_sub(1));
    }

    resp.json::<serde_json::Value>().await.map_err(LiterLlmError::from)
}

/// Send a GET request and return the raw response bytes.
///
/// Used for endpoints that return binary data (e.g. GET /files/{id}/content
/// for downloading file contents).
///
/// Retries on 429 / 5xx according to `max_retries`.
#[tracing::instrument(
    skip_all,
    fields(
        http.method = "GET",
        http.url = %url,
        http.status_code = tracing::field::Empty,
        http.retry_count = tracing::field::Empty,
    )
)]
pub async fn get_binary(
    client: &reqwest::Client,
    url: &str,
    auth_header: Option<(&str, &str)>,
    extra_headers: &[(&str, &str)],
    max_retries: u32,
) -> Result<Bytes> {
    let mut retry_count = 0u32;

    let resp = with_retry(max_retries, || {
        let mut builder = client.get(url);
        if let Some((name, value)) = auth_header {
            builder = builder.header(name, value);
        }
        for (name, value) in extra_headers {
            builder = builder.header(*name, *value);
        }
        retry_count += 1;
        builder.send()
    })
    .await?;

    {
        let span = tracing::Span::current();
        span.record("http.status_code", resp.status().as_u16());
        span.record("http.retry_count", retry_count.saturating_sub(1));
    }

    resp.bytes().await.map_err(LiterLlmError::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Bind an ephemeral loopback port, then immediately release it. Connections to
    /// the now-closed port are refused instantly (no live network, no timeout wait),
    /// giving a deterministic, genuine `reqwest::Error` transport failure for tests.
    fn closed_port_url() -> String {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
        let port = listener.local_addr().expect("local_addr").port();
        drop(listener);
        format!("http://127.0.0.1:{port}/")
    }

    #[tokio::test]
    async fn with_retry_retries_transport_errors_before_giving_up() {
        // ~keep Regression test for #44: before the fix, `send().await?` propagated a
        // transport-level error (e.g. connection refused) immediately via `?`,
        // bypassing the retry loop entirely regardless of `max_retries`.
        let client = reqwest::Client::new();
        let url = closed_port_url();
        let mut attempts = 0u32;

        let result = with_retry(2, || {
            attempts += 1;
            client.get(url.as_str()).send()
        })
        .await;

        assert!(
            result.is_err(),
            "every attempt fails at the transport layer, so the final result must be Err"
        );
        assert_eq!(
            attempts, 3,
            "must attempt once plus 2 retries (matching max_retries) before giving up"
        );
    }

    #[tokio::test]
    async fn with_retry_does_not_retry_transport_errors_when_max_retries_is_zero() {
        let client = reqwest::Client::new();
        let url = closed_port_url();
        let mut attempts = 0u32;

        let result = with_retry(0, || {
            attempts += 1;
            client.get(url.as_str()).send()
        })
        .await;

        assert!(result.is_err());
        assert_eq!(
            attempts, 1,
            "max_retries = 0 must still make exactly one attempt and no retries"
        );
    }
}
