---
id: fixture_swift_basic_chat
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"messages\":[{\"content\":\"Say hello\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"temperature\":0}")
_ = try await LiterLlm.chat(request: _request)

```
