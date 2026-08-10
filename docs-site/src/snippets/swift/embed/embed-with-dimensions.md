---
id: fixture_swift_embed_with_dimensions
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.embeddingRequestFromJson("{\"dimensions\":256,\"input\":\"Hello world\",\"model\":\"text-embedding-3-small\"}")
_ = try await LiterLlm.embed(request: _request)

```
