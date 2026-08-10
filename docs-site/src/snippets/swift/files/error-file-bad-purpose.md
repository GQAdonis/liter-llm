---
id: fixture_swift_error_file_bad_purpose
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

do {
    let _request = try LiterLlm.createFileRequestFromJson("{\"file\":\"data.jsonl\",\"purpose\":\"invalid-purpose\"}")
    _ = try await LiterLlm.createFile(request: _request)
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
