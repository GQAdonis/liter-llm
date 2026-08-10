---
id: fixture_swift_smoke_rerank_basic
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.rerankRequestFromJson("{\"documents\":[\"Machine learning is a subset of AI.\",\"The weather is sunny today.\",\"Deep learning uses neural networks.\"],\"model\":\"rerank-v3.5\",\"query\":\"What is machine learning?\"}")
_ = try await LiterLlm.rerank(request: _request)

```
