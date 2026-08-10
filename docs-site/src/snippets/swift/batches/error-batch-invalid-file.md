---
id: fixture_swift_error_batch_invalid_file
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

do {
    let _request = try LiterLlm.createBatchRequestFromJson("{\"completion_window\":\"24h\",\"endpoint\":\"/v1/chat/completions\",\"input_file_id\":\"file-wrong-purpose\"}")
    _ = try await LiterLlm.createBatch(request: _request)
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
