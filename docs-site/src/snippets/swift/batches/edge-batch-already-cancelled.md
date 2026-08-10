---
id: fixture_swift_edge_batch_already_cancelled
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

do {
    _ = try await LiterLlm.cancelBatch(batchId: "batch-cancelled001")
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
