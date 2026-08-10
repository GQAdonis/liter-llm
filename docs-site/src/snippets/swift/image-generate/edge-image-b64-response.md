---
id: fixture_swift_edge_image_b64_response
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.createImageRequestFromJson("{\"model\":\"dall-e-3\",\"n\":1,\"prompt\":\"A blue circle\",\"response_format\":\"b64_json\",\"size\":\"1024x1024\"}")
_ = try await LiterLlm.imageGenerate(request: _request)

```
