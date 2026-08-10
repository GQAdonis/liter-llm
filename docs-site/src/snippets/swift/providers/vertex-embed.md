---
id: fixture_swift_vertex_embed
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.embeddingRequestFromJson("{\"input\":\"Hello\",\"model\":\"vertex_ai/text-embedding-005\"}")
_ = try await LiterLlm.embed(request: _request)

```
