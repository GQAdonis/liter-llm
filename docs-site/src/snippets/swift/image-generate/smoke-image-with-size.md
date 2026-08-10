---
id: fixture_swift_smoke_image_with_size
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.createImageRequestFromJson("{\"model\":\"dall-e-3\",\"n\":1,\"prompt\":\"A sunset over mountains\",\"size\":\"1792x1024\"}")
_ = try await LiterLlm.imageGenerate(request: _request)

```
