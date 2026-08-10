---
id: fixture_swift_smoke_retrieve_file
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

_ = try await LiterLlm.retrieveFile(fileId: "file-abc123")

```
