---
id: fixture_swift_binding_api_parity
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"messages\":[{\"content\":\"Contract test\",\"role\":\"user\"}],\"model\":\"openai/gpt-4o\"}")
_ = try await LiterLlm.chat(request: _request)

```
