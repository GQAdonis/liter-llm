---
id: fixture_swift_local_stream_ollama
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"messages\":[{\"content\":\"Count to 3\",\"role\":\"user\"}],\"model\":\"ollama/qwen2:0.5b\",\"stream\":true}")
let result = try await LiterLlm.chatStream(request: _request)
var chunks: [LiterLlm.ChatCompletionChunk] = []
        for try await _chunk in result { chunks.append(_chunk) }

```
