---
id: fixture_swift_local_provider_ollama
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"messages\":[{\"content\":\"Hello\",\"role\":\"user\"}],\"model\":\"ollama/qwen2:0.5b\"}")
_ = try await LiterLlm.chat(request: _request)

```
