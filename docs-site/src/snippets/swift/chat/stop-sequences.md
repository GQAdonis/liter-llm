---
id: fixture_swift_stop_sequences
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"messages\":[{\"content\":\"List items until you see STOP\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"stop\":[\"STOP\",\"END\"]}")
_ = try await LiterLlm.chat(request: _request)

```
