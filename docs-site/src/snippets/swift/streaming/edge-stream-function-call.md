---
id: fixture_swift_edge_stream_function_call
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"messages\":[{\"content\":\"What's the weather?\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"tools\":[{\"function\":{\"name\":\"get_weather\",\"parameters\":{\"properties\":{\"city\":{\"type\":\"string\"}},\"type\":\"object\"}},\"type\":\"function\"}]}")
let result = try await LiterLlm.chatStream(request: _request)
var chunks: [LiterLlm.ChatCompletionChunk] = []
        for try await _chunk in result { chunks.append(_chunk) }

```
