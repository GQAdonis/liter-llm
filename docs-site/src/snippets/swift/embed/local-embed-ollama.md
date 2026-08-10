---
id: fixture_swift_local_embed_ollama
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.embeddingRequestFromJson("{\"input\":\"The quick brown fox jumps over the lazy dog\",\"model\":\"ollama/all-minilm\"}")
_ = try await LiterLlm.embed(request: _request)

```
