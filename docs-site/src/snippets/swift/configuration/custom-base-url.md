---
id: fixture_swift_custom_base_url
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"messages\":[{\"content\":\"Hello\",\"role\":\"user\"}],\"model\":\"local-model\"}")
_ = try await LiterLlm.chat(request: _request)

```
