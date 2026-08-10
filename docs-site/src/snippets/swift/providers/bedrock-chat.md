---
id: fixture_swift_bedrock_chat
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"max_tokens\":16,\"messages\":[{\"content\":\"Say hello in one word.\",\"role\":\"user\"}],\"model\":\"bedrock/anthropic.claude-3-sonnet-20240229-v1:0\",\"temperature\":0}")
_ = try await LiterLlm.chat(request: _request)

```
