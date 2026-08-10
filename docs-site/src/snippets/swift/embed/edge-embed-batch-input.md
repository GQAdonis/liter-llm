---
id: fixture_swift_edge_embed_batch_input
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.embeddingRequestFromJson("{\"input\":[\"Hello world\",\"Goodbye world\"],\"model\":\"text-embedding-3-small\"}")
_ = try await LiterLlm.embed(request: _request)

```
