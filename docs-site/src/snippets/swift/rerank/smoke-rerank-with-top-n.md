---
id: fixture_swift_smoke_rerank_with_top_n
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.rerankRequestFromJson("{\"documents\":[\"Python is a programming language.\",\"Cats are cute animals.\",\"Python was created by Guido van Rossum.\",\"The sun is a star.\"],\"model\":\"rerank-v3.5\",\"query\":\"What is Python?\",\"top_n\":2}")
_ = try await LiterLlm.rerank(request: _request)

```
