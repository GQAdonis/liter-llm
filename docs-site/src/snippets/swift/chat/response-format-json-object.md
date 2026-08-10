---
id: fixture_swift_response_format_json_object
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"messages\":[{\"content\":\"Respond with JSON only.\",\"role\":\"system\"},{\"content\":\"Give me a user object with name and age fields.\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"response_format\":{\"type\":\"json_object\"}}")
_ = try await LiterLlm.chat(request: _request)

```
