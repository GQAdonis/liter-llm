//! OpenDAL-backed vector store.
//!
//! Persists vectors as JSON entries via any OpenDAL backend (filesystem, S3,
//! Redis, GCS, etc.).  Each entry is stored under `{prefix}{id}` with a JSON
//! payload containing the vector, metadata, and insertion timestamp.
//!
//! This implementation performs a full scan (list + read) for every
//! [`VectorStore::search`] call, which makes it suitable only for corpora
//! where the data set fits comfortably in memory during the scan.  For larger
//! data sets, use a dedicated ANN index (Qdrant, pgvector) via the
//! [`VectorStore`] trait.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use opendal::Operator;
use serde::{Deserialize, Serialize};

use super::{VectorMatch, VectorMetadata, VectorStore, tenant_matches};
use crate::error::{LiterLlmError, Result};

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    dot / (norm_a * norm_b)
}

#[derive(Serialize, Deserialize)]
struct StoredVector {
    vec: Vec<f32>,
    cache_key: u64,
    /// Serialized request body stored at insertion time.
    ///
    /// Used by the semantic cache tier's collision-guard check: when a vector
    /// match is found, the tier passes this body to `CacheStore::get` so that
    /// the body comparison succeeds even though the current request differs.
    #[serde(default)]
    original_request_body: String,
    tenant_id: Option<String>,
    /// Unix timestamp (seconds) of insertion.
    inserted_at_secs: u64,
    extra: HashMap<String, String>,
}

impl StoredVector {
    fn into_metadata(self) -> VectorMetadata {
        VectorMetadata {
            cache_key: self.cache_key,
            original_request_body: self.original_request_body,
            tenant_id: self.tenant_id,
            inserted_at: UNIX_EPOCH + Duration::from_secs(self.inserted_at_secs),
            extra: self.extra,
        }
    }
}

/// Vector store backed by an [`opendal::Operator`].
///
/// Each vector is stored as a JSON file at `{prefix}{id}`.  Queries perform a
/// full-scan list + read; this is appropriate for small to medium corpora.
pub struct OpenDalVectorStore {
    operator: Operator,
    prefix: String,
    dim: usize,
}

impl OpenDalVectorStore {
    /// Create a new OpenDAL vector store.
    ///
    /// `prefix` is prepended to all entry keys (e.g. `"vec-cache/"`).
    /// `dim` is the expected vector dimensionality.
    #[must_use]
    pub fn new(operator: Operator, prefix: impl Into<String>, dim: usize) -> Self {
        Self {
            operator,
            prefix: prefix.into(),
            dim,
        }
    }

    fn entry_path(&self, id: &str) -> String {
        format!("{}{}", self.prefix, id)
    }

    /// Convert a [`SystemTime`] to a Unix timestamp in whole seconds.
    ///
    /// Clamps to `0` when `time` is before the epoch instead of failing the
    /// whole write, since insertion timestamps are best-effort metadata.
    fn to_unix_secs(time: SystemTime) -> u64 {
        time.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
    }
}

impl VectorStore for OpenDalVectorStore {
    fn search<'a>(
        &'a self,
        query_vec: &'a [f32],
        k: usize,
        threshold: f32,
        tenant_id: Option<&'a str>,
    ) -> Pin<Box<dyn Future<Output = Vec<VectorMatch>> + Send + 'a>> {
        Box::pin(async move {
            let entries = match self.operator.list(&self.prefix).await {
                Ok(e) => e,
                Err(error) => {
                    tracing::warn!(
                        prefix = %self.prefix,
                        %error,
                        "vector store: listing entries failed; search degraded to empty result"
                    );
                    return Vec::new();
                }
            };

            let mut matches = Vec::new();
            for entry in entries {
                let path = entry.path().to_owned();
                let bytes = match self.operator.read(&path).await {
                    Ok(b) => b,
                    Err(error) => {
                        tracing::warn!(%path, %error, "vector store: reading entry failed; skipping entry");
                        continue;
                    }
                };
                let stored: StoredVector = match serde_json::from_slice(bytes.to_bytes().as_ref()) {
                    Ok(s) => s,
                    Err(error) => {
                        tracing::warn!(
                            %path,
                            %error,
                            "vector store: entry contains invalid JSON; skipping corrupt entry"
                        );
                        continue;
                    }
                };
                if !tenant_matches(stored.tenant_id.as_deref(), tenant_id) {
                    continue;
                }
                let sim = cosine_similarity(query_vec, &stored.vec);
                if sim >= threshold {
                    let id = path.strip_prefix(&self.prefix).unwrap_or(&path).to_owned();
                    let metadata = stored.into_metadata();
                    matches.push(VectorMatch {
                        id,
                        similarity: sim,
                        metadata,
                    });
                }
            }

            matches.sort_by(|a, b| {
                b.similarity
                    .partial_cmp(&a.similarity)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            matches.truncate(k);
            matches
        })
    }

    fn upsert<'a>(
        &'a self,
        id: String,
        vec: Vec<f32>,
        metadata: VectorMetadata,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        if vec.len() != self.dim {
            return Box::pin(std::future::ready(Err(LiterLlmError::InternalError {
                message: format!(
                    "vector dimension mismatch: store expects {} but received {}",
                    self.dim,
                    vec.len()
                ),
            })));
        }
        Box::pin(async move {
            let path = self.entry_path(&id);
            let stored = StoredVector {
                vec,
                cache_key: metadata.cache_key,
                original_request_body: metadata.original_request_body,
                tenant_id: metadata.tenant_id,
                inserted_at_secs: Self::to_unix_secs(metadata.inserted_at),
                extra: metadata.extra,
            };
            let bytes = serde_json::to_vec(&stored).map_err(|e| LiterLlmError::InternalError {
                message: format!("vector store: serialization failed: {e}"),
            })?;
            self.operator
                .write(&path, bytes)
                .await
                .map(|_| ())
                .map_err(|e| LiterLlmError::InternalError {
                    message: format!("vector store: write failed for '{path}': {e}"),
                })
        })
    }

    fn delete<'a>(&'a self, id: &'a str) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            let path = self.entry_path(id);
            match self.operator.delete(&path).await {
                Ok(()) => Ok(()),
                Err(e) if e.kind() == opendal::ErrorKind::NotFound => Ok(()),
                Err(e) => Err(LiterLlmError::InternalError {
                    message: format!("vector store: delete failed for '{path}': {e}"),
                }),
            }
        })
    }

    fn dim(&self) -> usize {
        self.dim
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn memory_store(dim: usize) -> OpenDalVectorStore {
        let op = Operator::via_iter("memory", std::iter::empty::<(String, String)>())
            .expect("memory backend should always build");
        OpenDalVectorStore::new(op, "vec/", dim)
    }

    fn meta(cache_key: u64) -> VectorMetadata {
        VectorMetadata {
            cache_key,
            original_request_body: String::new(),
            tenant_id: None,
            inserted_at: SystemTime::now(),
            extra: HashMap::new(),
        }
    }

    #[tokio::test]
    async fn upsert_and_search_returns_match() {
        let store = memory_store(3);
        store.upsert("e1".into(), vec![1.0, 0.0, 0.0], meta(7)).await.unwrap();
        let results = store.search(&[1.0, 0.0, 0.0], 5, 0.99, None).await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "e1");
        assert_eq!(results[0].metadata.cache_key, 7);
    }

    #[tokio::test]
    async fn search_filters_below_threshold() {
        let store = memory_store(3);
        store.upsert("e1".into(), vec![1.0, 0.0, 0.0], meta(1)).await.unwrap();
        store.upsert("e2".into(), vec![0.0, 1.0, 0.0], meta(2)).await.unwrap();
        let results = store.search(&[1.0, 0.0, 0.0], 5, 0.9, None).await;
        assert_eq!(results.len(), 1, "orthogonal vector should be filtered");
    }

    #[tokio::test]
    async fn delete_removes_entry() {
        let store = memory_store(3);
        store.upsert("e1".into(), vec![1.0, 0.0, 0.0], meta(1)).await.unwrap();
        store.delete("e1").await.unwrap();
        let results = store.search(&[1.0, 0.0, 0.0], 5, 0.0, None).await;
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn delete_nonexistent_is_noop() {
        let store = memory_store(3);
        let result = store.delete("does-not-exist").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn upsert_dimension_mismatch_returns_error() {
        let store = memory_store(3);
        let result = store.upsert("bad".into(), vec![1.0, 0.0], meta(1)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn upsert_preserves_caller_supplied_inserted_at() {
        let store = memory_store(3);
        let inserted_at = UNIX_EPOCH + Duration::from_secs(1_700_000_000);
        let mut metadata = meta(1);
        metadata.inserted_at = inserted_at;
        store.upsert("e1".into(), vec![1.0, 0.0, 0.0], metadata).await.unwrap();

        let results = store.search(&[1.0, 0.0, 0.0], 5, 0.99, None).await;
        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0].metadata.inserted_at, inserted_at,
            "upsert must persist the caller-supplied inserted_at instead of overwriting it with now()"
        );
    }

    #[tokio::test]
    async fn search_skips_entry_with_invalid_json_without_failing() {
        let store = memory_store(3);
        store.upsert("good".into(), vec![1.0, 0.0, 0.0], meta(1)).await.unwrap();
        let corrupt_path = store.entry_path("corrupt");
        store.operator.write(&corrupt_path, b"not json".to_vec()).await.unwrap();

        let results = store.search(&[1.0, 0.0, 0.0], 5, 0.0, None).await;
        assert_eq!(results.len(), 1, "corrupt entry must be skipped, not crash the search");
        assert_eq!(results[0].id, "good");
    }

    fn meta_for_tenant(cache_key: u64, tenant_id: &str) -> VectorMetadata {
        VectorMetadata {
            tenant_id: Some(tenant_id.to_owned()),
            ..meta(cache_key)
        }
    }

    /// A tenant-scoped search must never return another tenant's entry, even
    /// when the vector similarity would otherwise match above threshold.
    #[tokio::test]
    async fn search_excludes_entries_from_other_tenants() {
        let store = memory_store(3);
        store
            .upsert("a".into(), vec![1.0, 0.0, 0.0], meta_for_tenant(1, "tenant-a"))
            .await
            .unwrap();

        let results = store.search(&[1.0, 0.0, 0.0], 5, 0.99, Some("tenant-b")).await;
        assert!(
            results.is_empty(),
            "tenant-b must not see tenant-a's semantically-matched entry"
        );

        let results = store.search(&[1.0, 0.0, 0.0], 5, 0.99, Some("tenant-a")).await;
        assert_eq!(results.len(), 1, "tenant-a must still see its own entry");
    }

    /// A `None`-tenant query must not match a tenant-scoped entry, and a
    /// tenant-scoped query must not match a `None`-tenant entry.
    #[tokio::test]
    async fn search_tenant_none_is_not_a_wildcard() {
        let store = memory_store(3);
        store
            .upsert("scoped".into(), vec![1.0, 0.0, 0.0], meta_for_tenant(1, "tenant-a"))
            .await
            .unwrap();
        store
            .upsert("unscoped".into(), vec![1.0, 0.0, 0.0], meta(2))
            .await
            .unwrap();

        let results = store.search(&[1.0, 0.0, 0.0], 5, 0.99, None).await;
        assert_eq!(results.len(), 1, "None query must only match the None-tenant entry");
        assert_eq!(results[0].id, "unscoped");

        let results = store.search(&[1.0, 0.0, 0.0], 5, 0.99, Some("tenant-a")).await;
        assert_eq!(
            results.len(),
            1,
            "tenant-scoped query must only match that tenant's entry"
        );
        assert_eq!(results[0].id, "scoped");
    }
}
