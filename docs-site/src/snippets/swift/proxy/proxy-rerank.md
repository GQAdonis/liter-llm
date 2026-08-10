---
id: fixture_swift_proxy_rerank
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.rerankRequestFromJson("{\"documents\":[\"Deep learning is a subset of machine learning using neural networks.\",\"The stock market closed higher today.\"],\"model\":\"rerank-v3.5\",\"query\":\"What is deep learning?\"}")
_ = try await LiterLlm.rerank(request: _request)

```
