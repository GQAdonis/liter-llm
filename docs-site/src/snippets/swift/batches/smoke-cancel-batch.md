---
id: fixture_swift_smoke_cancel_batch
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

_ = try await LiterLlm.cancelBatch(batchId: "batch-def456")

```
