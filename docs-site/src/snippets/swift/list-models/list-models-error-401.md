---
id: fixture_swift_list_models_error_401
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

do {
    _ = try await LiterLlm.listModels()
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
