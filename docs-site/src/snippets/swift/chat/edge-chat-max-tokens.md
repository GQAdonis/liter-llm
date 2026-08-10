---
id: fixture_swift_edge_chat_max_tokens
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"max_tokens\":1,\"messages\":[{\"content\":\"Write a story\",\"role\":\"user\"}],\"model\":\"gpt-4\"}")
_ = try await LiterLlm.chat(request: _request)

```
