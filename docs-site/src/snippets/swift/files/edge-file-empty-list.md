---
id: fixture_swift_edge_file_empty_list
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

_ = try await LiterLlm.listFiles(nil)

```
