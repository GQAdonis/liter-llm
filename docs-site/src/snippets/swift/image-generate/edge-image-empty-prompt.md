---
id: fixture_swift_edge_image_empty_prompt
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

do {
    let _request = try LiterLlm.createImageRequestFromJson("{\"model\":\"dall-e-3\",\"n\":1,\"prompt\":\"\",\"size\":\"1024x1024\"}")
    _ = try await LiterLlm.imageGenerate(request: _request)
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
