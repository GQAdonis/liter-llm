---
id: fixture_swift_response_format_json_schema
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"messages\":[{\"content\":\"What is the temperature in Paris today?\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"response_format\":{\"json_schema\":{\"name\":\"weather\",\"schema\":{\"properties\":{\"temp\":{\"type\":\"number\"}},\"required\":[\"temp\"],\"type\":\"object\"}},\"type\":\"json_schema\"}}")
_ = try await LiterLlm.chat(request: _request)

```
