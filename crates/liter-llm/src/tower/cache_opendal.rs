//! OpenDAL-backed cache store for the response cache.
//!
//! Implements [`CacheStore`] using an [`opendal::Operator`] for persistence.
//! Supports any OpenDAL backend (S3, Redis, GCS, local filesystem, etc.).

use std::collections::{HashMap, VecDeque};
use std::future::Future;
use std::pin::Pin;
use std::sync::RwLock;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use opendal::Operator;
use serde::{Deserialize, Serialize};

use super::cache::{CacheStore, CachedResponse};

/// A cached entry stored via OpenDAL, including metadata for TTL and
/// collision detection.
#[derive(Serialize, Deserialize)]
struct StoredEntry {
    request_body: String,
    response: CachedResponse,
    /// Unix timestamp (seconds) when this entry expires.
    expires_at: u64,
}

/// Cache store backed by an [`opendal::Operator`].
///
/// Entries are stored as JSON files under `{prefix}/{key}`. TTL is embedded
/// in the stored entry and checked on read. Backend failures are non-fatal:
/// they log a warning and behave as a cache miss / no-op.
///
/// # Bounded growth
///
/// Unlike [`super::cache::InMemoryStore`] (which evicts on `max_entries`),
/// this store previously grew without bound: nothing ever deleted an entry
/// except its own TTL expiry, and expiry is checked lazily on `get` — a key
/// that is written once and never read again lives in the backend forever.
/// For a remote object store (S3, GCS) this is often acceptable (or even
/// desired), but for a local `fs`/`memory` backend it is an unbounded disk/
/// memory leak. Set `max_entries` via [`Self::with_max_entries`] to cap the
/// number of live keys with LRU-by-insertion-order eviction, tracked by an
/// in-process index (`order`). The index is best-effort and process-local:
/// entries written by a different process (e.g. another replica sharing the
/// same backend) are not counted against this instance's cap.
pub struct OpenDalCacheStore {
    operator: Operator,
    prefix: String,
    ttl: Duration,
    max_entries: Option<usize>,
    /// Insertion-ordered keys, front = oldest. Only populated/consulted when
    /// `max_entries` is `Some`.
    order: RwLock<VecDeque<u64>>,
}

impl OpenDalCacheStore {
    /// Create a new OpenDAL cache store with unbounded entry count.
    ///
    /// `operator` must be a fully configured OpenDAL operator.
    /// `prefix` is prepended to all cache keys (e.g. `"llm-cache/"`).
    /// `ttl` controls how long entries are valid.
    pub fn new(operator: Operator, prefix: impl Into<String>, ttl: Duration) -> Self {
        Self {
            operator,
            prefix: prefix.into(),
            ttl,
            max_entries: None,
            order: RwLock::new(VecDeque::new()),
        }
    }

    /// Cap the number of live entries this instance will keep, evicting the
    /// oldest-inserted key (by this instance's own insertion order, not
    /// backend mtime) once the cap is exceeded. See the type-level docs for
    /// the process-local caveat.
    #[must_use]
    pub fn with_max_entries(mut self, max_entries: usize) -> Self {
        self.max_entries = Some(max_entries);
        self
    }

    /// Build an OpenDAL operator from a scheme name and config map.
    ///
    /// # Errors
    /// Returns an error if the scheme is unknown or the config is invalid.
    pub fn from_config(
        scheme: &str,
        config: HashMap<String, String>,
        prefix: impl Into<String>,
        ttl: Duration,
    ) -> crate::error::Result<Self> {
        let operator = Operator::via_iter(scheme, config).map_err(|e| crate::error::LiterLlmError::InternalError {
            message: format!("failed to build OpenDAL operator for '{scheme}': {e}"),
        })?;
        Ok(Self::new(operator, prefix, ttl))
    }

    fn key_path(&self, key: u64) -> String {
        format!("{}{key}", self.prefix)
    }

    fn now_secs() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    /// Record `key` as the most-recently-inserted entry and return any keys
    /// evicted by exceeding `max_entries`. No-op (returns an empty `Vec`)
    /// when `max_entries` is `None`.
    fn touch_and_evict(&self, key: u64) -> Vec<u64> {
        let Some(max_entries) = self.max_entries else {
            return Vec::new();
        };

        let Ok(mut order) = self.order.write() else {
            tracing::warn!("OpenDAL cache: order-tracking lock poisoned; max_entries eviction disabled for this write");
            return Vec::new();
        };

        order.retain(|k| *k != key);
        order.push_back(key);

        let mut evicted = Vec::new();
        while order.len() > max_entries {
            if let Some(oldest) = order.pop_front() {
                evicted.push(oldest);
            } else {
                break;
            }
        }
        evicted
    }

    /// Drop `key` from the insertion-order index (called on explicit `remove`).
    fn forget(&self, key: u64) {
        if let Ok(mut order) = self.order.write() {
            order.retain(|k| *k != key);
        } else {
            tracing::warn!("OpenDAL cache: order-tracking lock poisoned; could not forget removed key");
        }
    }
}

impl CacheStore for OpenDalCacheStore {
    fn get(&self, key: u64, request_body: &str) -> Pin<Box<dyn Future<Output = Option<CachedResponse>> + Send + '_>> {
        let path = self.key_path(key);
        let request_body = request_body.to_owned();
        Box::pin(async move {
            let bytes = match self.operator.read(&path).await {
                Ok(b) => b,
                // ~keep NotFound is an expected, silent cache miss; any other error indicates a
                // ~keep degraded backend and must be surfaced, or misses are indistinguishable from outages.
                Err(e) if e.kind() == opendal::ErrorKind::NotFound => return None,
                Err(e) => {
                    tracing::warn!("OpenDAL cache: failed to read {path}: {e}");
                    return None;
                }
            };
            let entry: StoredEntry = match serde_json::from_slice(bytes.to_bytes().as_ref()) {
                Ok(e) => e,
                Err(e) => {
                    tracing::warn!("OpenDAL cache: failed to deserialize entry at {path}: {e}");
                    return None;
                }
            };
            if Self::now_secs() > entry.expires_at {
                if let Err(e) = self.operator.delete(&path).await {
                    tracing::warn!("OpenDAL cache: failed to delete expired entry {path}: {e}");
                }
                self.forget(key);
                return None;
            }
            if entry.request_body != request_body {
                return None;
            }
            Some(entry.response)
        })
    }

    fn put(
        &self,
        key: u64,
        request_body: String,
        response: CachedResponse,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        let path = self.key_path(key);
        let entry = StoredEntry {
            request_body,
            response,
            expires_at: Self::now_secs() + self.ttl.as_secs(),
        };
        Box::pin(async move {
            let bytes = match serde_json::to_vec(&entry) {
                Ok(b) => b,
                Err(e) => {
                    tracing::warn!("OpenDAL cache: failed to serialize entry: {e}");
                    return;
                }
            };
            if let Err(e) = self.operator.write(&path, bytes).await {
                tracing::warn!("OpenDAL cache: failed to write {path}: {e}");
                return;
            }

            for evicted_key in self.touch_and_evict(key) {
                let evicted_path = self.key_path(evicted_key);
                if let Err(e) = self.operator.delete(&evicted_path).await {
                    tracing::warn!("OpenDAL cache: failed to delete evicted entry {evicted_path}: {e}");
                }
            }
        })
    }

    fn remove(&self, key: u64) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        let path = self.key_path(key);
        Box::pin(async move {
            if let Err(e) = self.operator.delete(&path).await {
                tracing::warn!("OpenDAL cache: failed to delete {path}: {e}");
            }
            self.forget(key);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tower::cache::{CacheStore, CachedResponse};
    use crate::types::{AssistantMessage, ChatCompletionResponse, Choice, FinishReason};

    fn memory_store(ttl_secs: u64) -> OpenDalCacheStore {
        let op = Operator::via_iter("memory", std::iter::empty::<(String, String)>())
            .expect("memory backend should always build");
        OpenDalCacheStore::new(op, "test/", Duration::from_secs(ttl_secs))
    }

    fn dummy_response() -> CachedResponse {
        CachedResponse::Chat(ChatCompletionResponse {
            id: "test-resp-001".into(),
            object: "chat.completion".into(),
            created: 1_700_000_000,
            model: "gpt-4".into(),
            choices: vec![Choice {
                index: 0,
                message: AssistantMessage {
                    content: Some("Hello!".into()),
                    name: None,
                    tool_calls: None,
                    refusal: None,
                    function_call: None,
                    reasoning_content: None,
                },
                finish_reason: Some(FinishReason::Stop),
            }],
            usage: None,
            system_fingerprint: None,
            service_tier: None,
        })
    }

    #[tokio::test]
    async fn put_and_get_round_trip() {
        let store = memory_store(300);
        store.put(42, "request-body-a".into(), dummy_response()).await;
        let cached = store.get(42, "request-body-a").await;
        assert!(cached.is_some(), "expected a cached response after put");
        match cached.expect("cached value should be present") {
            CachedResponse::Chat(resp) => {
                assert_eq!(resp.id, "test-resp-001");
                assert_eq!(resp.model, "gpt-4");
            }
            _ => panic!("expected CachedResponse::Chat variant"),
        }
    }

    #[tokio::test]
    async fn get_returns_none_for_missing_key() {
        let store = memory_store(300);
        let result = store.get(999, "any-body").await;
        assert!(result.is_none(), "expected None for a key that was never stored");
    }

    #[tokio::test]
    async fn get_returns_none_for_wrong_request_body() {
        let store = memory_store(300);
        store.put(1, "body-alpha".into(), dummy_response()).await;
        let result = store.get(1, "body-beta").await;
        assert!(result.is_none(), "expected None when request body does not match");
    }

    #[tokio::test]
    async fn expired_entry_returns_none() {
        let store = memory_store(0);
        store.put(1, "req".into(), dummy_response()).await;
        tokio::time::sleep(Duration::from_millis(1100)).await;
        let result = store.get(1, "req").await;
        assert!(result.is_none(), "expected None for expired entry");
    }

    #[tokio::test]
    async fn remove_deletes_entry() {
        let store = memory_store(300);
        store.put(7, "req".into(), dummy_response()).await;
        assert!(store.get(7, "req").await.is_some());
        store.remove(7).await;
        assert!(store.get(7, "req").await.is_none(), "expected None after remove");
    }

    #[tokio::test]
    async fn overwrite_replaces_previous_entry() {
        let store = memory_store(300);
        store.put(1, "req".into(), dummy_response()).await;

        let replacement = CachedResponse::Chat(ChatCompletionResponse {
            id: "test-resp-002".into(),
            object: "chat.completion".into(),
            created: 1_700_000_001,
            model: "gpt-4o".into(),
            choices: vec![],
            usage: None,
            system_fingerprint: None,
            service_tier: None,
        });
        store.put(1, "req".into(), replacement).await;

        match store.get(1, "req").await {
            Some(CachedResponse::Chat(resp)) => assert_eq!(resp.id, "test-resp-002"),
            _ => panic!("expected updated CachedResponse::Chat variant"),
        }
    }

    #[test]
    fn from_config_rejects_unknown_scheme() {
        let result = OpenDalCacheStore::from_config(
            "nonexistent_backend_xyz",
            std::collections::HashMap::new(),
            "prefix/",
            Duration::from_secs(60),
        );
        assert!(result.is_err(), "expected error for unknown scheme");
    }

    /// Regression for "unbounded opendal cache": without `max_entries`, keys
    /// accumulate in the backend forever unless individually read past their
    /// TTL. With `max_entries` set, inserting past the cap must evict the
    /// oldest-inserted key.
    #[tokio::test]
    async fn with_max_entries_evicts_oldest_key_on_overflow() {
        let op = Operator::via_iter("memory", std::iter::empty::<(String, String)>())
            .expect("memory backend should always build");
        let store = OpenDalCacheStore::new(op, "test/", Duration::from_secs(300)).with_max_entries(2);

        store.put(1, "req-1".into(), dummy_response()).await;
        store.put(2, "req-2".into(), dummy_response()).await;
        store.put(3, "req-3".into(), dummy_response()).await;

        assert!(
            store.get(1, "req-1").await.is_none(),
            "oldest key must be evicted once max_entries is exceeded"
        );
        assert!(store.get(2, "req-2").await.is_some(), "key 2 must still be present");
        assert!(store.get(3, "req-3").await.is_some(), "key 3 must still be present");
    }

    /// Re-inserting an existing key must refresh its position instead of
    /// evicting it as if it were the oldest entry.
    #[tokio::test]
    async fn with_max_entries_reinsert_refreshes_recency() {
        let op = Operator::via_iter("memory", std::iter::empty::<(String, String)>())
            .expect("memory backend should always build");
        let store = OpenDalCacheStore::new(op, "test/", Duration::from_secs(300)).with_max_entries(2);

        store.put(1, "req-1".into(), dummy_response()).await;
        store.put(2, "req-2".into(), dummy_response()).await;
        // Re-insert key 1: it is now the most-recently-inserted, so key 2 should be evicted next.
        store.put(1, "req-1".into(), dummy_response()).await;
        store.put(3, "req-3".into(), dummy_response()).await;

        assert!(store.get(1, "req-1").await.is_some(), "refreshed key must survive");
        assert!(
            store.get(2, "req-2").await.is_none(),
            "key 2 must be evicted as the true oldest after key 1 was refreshed"
        );
        assert!(store.get(3, "req-3").await.is_some(), "key 3 must still be present");
    }
}
