---
id: fixture_swift_azure_embed
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.embeddingRequestFromJson("{\"input\":\"Hello world\",\"model\":\"azure/text-embedding-ada-002\"}")
_ = try await LiterLlm.embed(request: _request)

```
