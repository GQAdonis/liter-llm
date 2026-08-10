---
id: fixture_swift_smoke_chat_anthropic
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"max_tokens\":10,\"messages\":[{\"content\":\"Say hello in exactly one word.\",\"role\":\"user\"}],\"model\":\"anthropic/claude-sonnet-4-20250514\"}")
_ = try await LiterLlm.chat(request: _request)

```
