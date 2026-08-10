---
id: fixture_swift_smoke_cache_memory
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"max_tokens\":5,\"messages\":[{\"content\":\"What is 2+2? Answer with just the number.\",\"role\":\"user\"}],\"model\":\"openai/gpt-4o-mini\"}")
_ = try await LiterLlm.chat(request: _request)

```
