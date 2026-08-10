---
id: fixture_swift_vertex_chat
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"max_tokens\":16,\"messages\":[{\"content\":\"Say hello in one word.\",\"role\":\"user\"}],\"model\":\"vertex_ai/gemini-2.0-flash\",\"temperature\":0}")
_ = try await LiterLlm.chat(request: _request)

```
