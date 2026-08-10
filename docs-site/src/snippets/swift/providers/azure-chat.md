---
id: fixture_swift_azure_chat
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"max_tokens\":16,\"messages\":[{\"content\":\"Say hello\",\"role\":\"user\"}],\"model\":\"azure/gpt-4\",\"temperature\":0}")
_ = try await LiterLlm.chat(request: _request)

```
