---
id: fixture_swift_error_moderate_auth_401
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

do {
    let _request = try LiterLlm.moderationRequestFromJson("{\"input\":\"Hello\",\"model\":\"omni-moderation-latest\"}")
    _ = try await LiterLlm.moderate(request: _request)
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
