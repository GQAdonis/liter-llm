---
id: fixture_swift_anthropic_tool_calling
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"max_tokens\":256,\"messages\":[{\"content\":\"What is the weather in London?\",\"role\":\"user\"}],\"model\":\"anthropic/claude-3-5-sonnet-20241022\",\"tool_choice\":\"auto\",\"tools\":[{\"function\":{\"description\":\"Get the current weather for a given location\",\"name\":\"get_weather\",\"parameters\":{\"properties\":{\"location\":{\"description\":\"The city and country, e.g. London, UK\",\"type\":\"string\"},\"unit\":{\"description\":\"The temperature unit to use\",\"enum\":[\"celsius\",\"fahrenheit\"],\"type\":\"string\"}},\"required\":[\"location\"],\"type\":\"object\"}},\"type\":\"function\"}]}")
_ = try await LiterLlm.chat(request: _request)

```
