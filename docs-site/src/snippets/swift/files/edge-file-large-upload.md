---
id: fixture_swift_edge_file_large_upload
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.createFileRequestFromJson("{\"file\":\"eyJwcm9tcHQiOiAibGFyZ2UgdHJhaW5pbmcgZGF0YSJ9Cg==\",\"filename\":\"large_training_data.jsonl\",\"purpose\":\"fine-tune\"}")
_ = try await LiterLlm.createFile(request: _request)

```
