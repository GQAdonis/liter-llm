---
id: fixture_swift_edge_moderate_empty_input
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.moderationRequestFromJson("{\"input\":\"\",\"model\":\"omni-moderation-latest\"}")
_ = try await LiterLlm.moderate(request: _request)

```
