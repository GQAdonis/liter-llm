---
id: fixture_swift_developer_message
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"messages\":[{\"content\":\"You are a coding assistant. Always respond with concise code examples.\",\"role\":\"developer\"},{\"content\":\"How do I reverse a string in Python?\",\"role\":\"user\"}],\"model\":\"gpt-4\"}")
_ = try await LiterLlm.chat(request: _request)

```
