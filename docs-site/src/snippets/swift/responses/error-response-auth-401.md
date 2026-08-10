---
id: fixture_swift_error_response_auth_401
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

do {
    let _request = try LiterLlm.createResponseRequestFromJson("{\"input\":\"Hello\",\"model\":\"gpt-4o\"}")
    _ = try await LiterLlm.createResponse(request: _request)
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
