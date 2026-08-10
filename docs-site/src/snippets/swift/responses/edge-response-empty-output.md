---
id: fixture_swift_edge_response_empty_output
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.createResponseRequestFromJson("{\"input\":\"\",\"model\":\"gpt-4o\"}")
_ = try await LiterLlm.createResponse(request: _request)

```
