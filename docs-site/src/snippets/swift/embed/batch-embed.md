---
id: fixture_swift_batch_embed
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.embeddingRequestFromJson("{\"input\":[\"Hello\",\"World\"],\"model\":\"text-embedding-3-small\"}")
_ = try await LiterLlm.embed(request: _request)

```
