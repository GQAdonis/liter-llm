---
id: fixture_swift_smoke_image_multiple
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.createImageRequestFromJson("{\"model\":\"dall-e-2\",\"n\":3,\"prompt\":\"A red bicycle\",\"size\":\"256x256\"}")
_ = try await LiterLlm.imageGenerate(request: _request)

```
