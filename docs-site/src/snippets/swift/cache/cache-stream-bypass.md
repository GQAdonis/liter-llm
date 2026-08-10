---
id: fixture_swift_cache_stream_bypass
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"messages\":[{\"content\":\"Hello\",\"role\":\"user\"}],\"model\":\"gpt-4\"}")
let result = try await LiterLlm.chatStream(request: _request)
var chunks: [LiterLlm.ChatCompletionChunk] = []
        for try await _chunk in result { chunks.append(_chunk) }

```
