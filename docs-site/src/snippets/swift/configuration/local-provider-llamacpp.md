---
id: fixture_swift_local_provider_llamacpp
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"messages\":[{\"content\":\"Hello\",\"role\":\"user\"}],\"model\":\"llamacpp/my-model\"}")
_ = try await LiterLlm.chat(request: _request)

```
