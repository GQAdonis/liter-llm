---
id: fixture_swift_parallel_tool_calls
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"messages\":[{\"content\":\"What is the weather in NYC and London?\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"parallel_tool_calls\":true,\"tools\":[{\"function\":{\"description\":\"Get the current weather for a given location\",\"name\":\"get_weather\",\"parameters\":{\"properties\":{\"location\":{\"description\":\"The city name\",\"type\":\"string\"}},\"required\":[\"location\"],\"type\":\"object\"}},\"type\":\"function\"}]}")
_ = try await LiterLlm.chat(request: _request)

```
