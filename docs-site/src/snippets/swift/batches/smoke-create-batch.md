---
id: fixture_swift_smoke_create_batch
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.createBatchRequestFromJson("{\"completion_window\":\"24h\",\"endpoint\":\"/v1/chat/completions\",\"input_file_id\":\"file-abc123\"}")
_ = try await LiterLlm.createBatch(request: _request)

```
