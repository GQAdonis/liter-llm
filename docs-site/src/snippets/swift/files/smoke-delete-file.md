---
id: fixture_swift_smoke_delete_file
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

_ = try await LiterLlm.deleteFile(fileId: "file-abc123")

```
