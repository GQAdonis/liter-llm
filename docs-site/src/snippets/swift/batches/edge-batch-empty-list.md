---
id: fixture_swift_edge_batch_empty_list
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

_ = try await LiterLlm.listBatches(nil)

```
