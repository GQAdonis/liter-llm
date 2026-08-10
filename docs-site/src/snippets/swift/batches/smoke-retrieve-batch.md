---
id: fixture_swift_smoke_retrieve_batch
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

_ = try await LiterLlm.retrieveBatch(batchId: "batch-abc123")

```
