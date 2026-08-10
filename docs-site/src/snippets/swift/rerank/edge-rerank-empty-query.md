---
id: fixture_swift_edge_rerank_empty_query
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.rerankRequestFromJson("{\"documents\":[\"Some document\",\"Another document\"],\"model\":\"rerank-v3.5\",\"query\":\"\"}")
_ = try await LiterLlm.rerank(request: _request)

```
