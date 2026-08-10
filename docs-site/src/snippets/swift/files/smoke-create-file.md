---
id: fixture_swift_smoke_create_file
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.createFileRequestFromJson("{\"file\":\"eyJwcm9tcHQiOiAiaGVsbG8ifQo=\",\"filename\":\"training_data.jsonl\",\"purpose\":\"fine-tune\"}")
_ = try await LiterLlm.createFile(request: _request)

```
