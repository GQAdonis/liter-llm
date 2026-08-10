---
id: fixture_swift_smoke_response_with_tools
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.createResponseRequestFromJson("{\"input\":\"What is the weather in San Francisco?\",\"model\":\"gpt-4o\",\"tools\":[{\"description\":\"Get current weather for a location\",\"name\":\"get_weather\",\"parameters\":{\"properties\":{\"location\":{\"type\":\"string\"}},\"required\":[\"location\"],\"type\":\"object\"},\"type\":\"function\"}]}")
_ = try await LiterLlm.createResponse(request: _request)

```
