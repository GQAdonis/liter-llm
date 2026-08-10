---
id: fixture_swift_proxy_image_generate
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.createImageRequestFromJson("{\"model\":\"dall-e-3\",\"n\":1,\"prompt\":\"A sunset over the ocean\",\"size\":\"1024x1024\"}")
_ = try await LiterLlm.imageGenerate(request: _request)

```
