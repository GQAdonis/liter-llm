---
id: fixture_swift_smoke_batch_completed
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

_ = try await LiterLlm.retrieveBatch(batchId: "batch-ghi789")

```
