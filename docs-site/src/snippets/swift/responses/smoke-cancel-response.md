---
id: fixture_swift_smoke_cancel_response
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

_ = try await LiterLlm.cancelResponse(responseId: "resp-def456")

```
