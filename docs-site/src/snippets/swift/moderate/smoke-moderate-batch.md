---
id: fixture_swift_smoke_moderate_batch
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.moderationRequestFromJson("{\"input\":[\"Hello world\",\"Nice weather today\"],\"model\":\"omni-moderation-latest\"}")
_ = try await LiterLlm.moderate(request: _request)

```
