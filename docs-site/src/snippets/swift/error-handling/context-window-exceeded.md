---
id: fixture_swift_context_window_exceeded
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

do {
    let _request = try LiterLlm.chatCompletionRequestFromJson("{\"messages\":[{\"content\":\"Very long prompt that exceeds the context window...\",\"role\":\"user\"}],\"model\":\"gpt-4\"}")
    _ = try await LiterLlm.chat(request: _request)
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
