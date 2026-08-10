---
id: fixture_swift_error_file_not_found
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

do {
    _ = try await LiterLlm.retrieveFile(fileId: "file-nonexistent")
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
