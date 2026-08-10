---
id: fixture_swift_embed_encoding_format
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.embeddingRequestFromJson("{\"encoding_format\":\"float\",\"input\":\"Test input\",\"model\":\"text-embedding-3-small\"}")
_ = try await LiterLlm.embed(request: _request)

```
