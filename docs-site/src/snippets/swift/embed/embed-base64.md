---
id: fixture_swift_embed_base64
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.embeddingRequestFromJson("{\"encoding_format\":\"base64\",\"input\":\"Test input\",\"model\":\"text-embedding-3-small\"}")
_ = try await LiterLlm.embed(request: _request)

```
