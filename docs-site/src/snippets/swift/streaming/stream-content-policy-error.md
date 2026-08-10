---
id: fixture_swift_stream_content_policy_error
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

do {
    let _request = try LiterLlm.chatCompletionRequestFromJson("{\"messages\":[{\"content\":\"Generate harmful content\",\"role\":\"user\"}],\"model\":\"gpt-4o\",\"stream\":true}")
    _ = try await LiterLlm.chatStream(request: _request)
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
