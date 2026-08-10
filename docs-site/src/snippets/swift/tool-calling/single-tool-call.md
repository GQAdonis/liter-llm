---
id: fixture_swift_single_tool_call
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"messages\":[{\"content\":\"What is the weather in San Francisco?\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"tool_choice\":\"auto\",\"tools\":[{\"function\":{\"description\":\"Get the current weather for a given location\",\"name\":\"get_weather\",\"parameters\":{\"properties\":{\"location\":{\"description\":\"The city and state, e.g. San Francisco, CA\",\"type\":\"string\"},\"unit\":{\"description\":\"The temperature unit to use\",\"enum\":[\"celsius\",\"fahrenheit\"],\"type\":\"string\"}},\"required\":[\"location\"],\"type\":\"object\"}},\"type\":\"function\"}]}")
_ = try await LiterLlm.chat(request: _request)

```
