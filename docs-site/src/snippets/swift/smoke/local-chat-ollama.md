---
id: fixture_swift_local_chat_ollama
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"max_tokens\":10,\"messages\":[{\"content\":\"Say hello in one word.\",\"role\":\"user\"}],\"model\":\"ollama/qwen2:0.5b\"}")
_ = try await LiterLlm.chat(request: _request)

```
