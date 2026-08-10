---
id: fixture_swift_multi_turn_conversation
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"messages\":[{\"content\":\"You are a helpful assistant.\",\"role\":\"system\"},{\"content\":\"What is 2 + 2?\",\"role\":\"user\"},{\"content\":\"2 + 2 equals 4.\",\"role\":\"assistant\"},{\"content\":\"And what is 4 + 4?\",\"role\":\"user\"}],\"model\":\"gpt-4\"}")
_ = try await LiterLlm.chat(request: _request)

```
