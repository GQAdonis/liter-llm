---
id: fixture_swift_smoke_moderate_flagged
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.moderationRequestFromJson("{\"input\":\"I want to hurt someone very badly\",\"model\":\"omni-moderation-latest\"}")
_ = try await LiterLlm.moderate(request: _request)

```
