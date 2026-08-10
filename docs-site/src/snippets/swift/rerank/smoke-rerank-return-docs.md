---
id: fixture_swift_smoke_rerank_return_docs
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.rerankRequestFromJson("{\"documents\":[\"Rust is a systems programming language.\",\"Iron rusts when exposed to water.\"],\"model\":\"rerank-v3.5\",\"query\":\"What is Rust?\",\"return_documents\":true}")
_ = try await LiterLlm.rerank(request: _request)

```
