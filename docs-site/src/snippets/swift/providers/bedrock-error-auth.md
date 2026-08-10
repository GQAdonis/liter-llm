---
id: fixture_swift_bedrock_error_auth
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

do {
    let _request = try LiterLlm.chatCompletionRequestFromJson("{\"messages\":[{\"content\":\"Hello\",\"role\":\"user\"}],\"model\":\"bedrock/anthropic.claude-3-sonnet-20240229-v1:0\"}")
    _ = try await LiterLlm.chat(request: _request)
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
