---
id: fixture_swift_smoke_streaming_openai
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"max_tokens\":50,\"messages\":[{\"content\":\"Count from 1 to 5.\",\"role\":\"user\"}],\"model\":\"openai/gpt-4o-mini\"}")
let result = try await LiterLlm.chatStream(request: _request)
var chunks: [LiterLlm.ChatCompletionChunk] = []
        for try await _chunk in result { chunks.append(_chunk) }

```
