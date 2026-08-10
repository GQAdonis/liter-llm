---
id: fixture_swift_error_file_auth_401
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

do {
    _ = try await LiterLlm.listFiles(nil)
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
