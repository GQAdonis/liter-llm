---
id: fixture_swift_smoke_provider_routing
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"max_tokens\":5,\"messages\":[{\"content\":\"Say hi.\",\"role\":\"user\"}],\"model\":\"openai/gpt-4o-mini\"}")
_ = try await LiterLlm.chat(request: _request)

```
