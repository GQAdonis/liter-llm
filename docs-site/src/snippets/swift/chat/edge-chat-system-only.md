---
id: fixture_swift_edge_chat_system_only
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"messages\":[{\"content\":\"You are a helpful and concise assistant\",\"role\":\"system\"},{\"content\":\"Hi\",\"role\":\"user\"}],\"model\":\"gpt-4\"}")
_ = try await LiterLlm.chat(request: _request)

```
