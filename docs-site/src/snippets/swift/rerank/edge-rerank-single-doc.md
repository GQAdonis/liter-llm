---
id: fixture_swift_edge_rerank_single_doc
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.rerankRequestFromJson("{\"documents\":[\"Artificial intelligence is the simulation of human intelligence.\"],\"model\":\"rerank-v3.5\",\"query\":\"What is AI?\"}")
_ = try await LiterLlm.rerank(request: _request)

```
