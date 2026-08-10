---
id: fixture_swift_vertex_error_auth
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

do {
    let _request = try LiterLlm.chatCompletionRequestFromJson("{\"messages\":[{\"content\":\"Hello\",\"role\":\"user\"}],\"model\":\"vertex_ai/gemini-2.0-flash\"}")
    _ = try await LiterLlm.chat(request: _request)
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
