---
id: fixture_swift_error_response_not_found
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

do {
    _ = try await LiterLlm.retrieveResponse(responseId: "resp-nonexistent")
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
