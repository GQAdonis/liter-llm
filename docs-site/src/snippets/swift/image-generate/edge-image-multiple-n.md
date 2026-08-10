---
id: fixture_swift_edge_image_multiple_n
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.createImageRequestFromJson("{\"model\":\"dall-e-3\",\"n\":3,\"prompt\":\"A cat\"}")
_ = try await LiterLlm.imageGenerate(request: _request)

```
