---
id: fixture_swift_tool_choice_specific
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"messages\":[{\"content\":\"What is the weather in Paris?\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"tool_choice\":{\"function\":{\"name\":\"get_weather\"},\"type\":\"function\"},\"tools\":[{\"function\":{\"description\":\"Get the current weather for a given location\",\"name\":\"get_weather\",\"parameters\":{\"properties\":{\"location\":{\"description\":\"The city name\",\"type\":\"string\"}},\"required\":[\"location\"],\"type\":\"object\"}},\"type\":\"function\"},{\"function\":{\"description\":\"Search the web for information\",\"name\":\"search_web\",\"parameters\":{\"properties\":{\"query\":{\"description\":\"The search query\",\"type\":\"string\"}},\"required\":[\"query\"],\"type\":\"object\"}},\"type\":\"function\"}]}")
_ = try await LiterLlm.chat(request: _request)

```
