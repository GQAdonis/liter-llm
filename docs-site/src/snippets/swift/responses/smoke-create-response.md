---
id: fixture_swift_smoke_create_response
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.createResponseRequestFromJson("{\"input\":\"Explain quantum computing in one sentence.\",\"model\":\"gpt-4o\"}")
_ = try await LiterLlm.createResponse(request: _request)

```
