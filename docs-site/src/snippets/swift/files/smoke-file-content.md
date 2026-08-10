---
id: fixture_swift_smoke_file_content
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

_ = try await LiterLlm.fileContent(fileId: "file-abc123")

```
