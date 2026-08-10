---
id: fixture_swift_vertex_stream
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"max_tokens\":32,\"messages\":[{\"content\":\"Count to three, one word per response.\",\"role\":\"user\"}],\"model\":\"vertex_ai/gemini-2.0-flash\",\"stream\":true}")
let result = try await LiterLlm.chatStream(request: _request)
var chunks: [LiterLlm.ChatCompletionChunk] = []
        for try await _chunk in result { chunks.append(_chunk) }

```
