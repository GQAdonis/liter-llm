---
id: fixture_swift_proxy_moderation
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.moderationRequestFromJson("{\"input\":\"The weather is nice today.\",\"model\":\"omni-moderation-latest\"}")
_ = try await LiterLlm.moderate(request: _request)

```
