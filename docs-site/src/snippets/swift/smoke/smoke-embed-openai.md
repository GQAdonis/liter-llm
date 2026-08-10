---
id: fixture_swift_smoke_embed_openai
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.embeddingRequestFromJson("{\"input\":[\"Hello world\"],\"model\":\"openai/text-embedding-3-small\"}")
_ = try await LiterLlm.embed(request: _request)

```
