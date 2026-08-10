---
id: fixture_swift_error_rerank_bad_request
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

do {
    let _request = try LiterLlm.rerankRequestFromJson("{\"documents\":[\"doc1\"],\"model\":\"nonexistent-rerank\",\"query\":\"test\"}")
    _ = try await LiterLlm.rerank(request: _request)
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
