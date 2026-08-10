---
id: fixture_swift_embed_error_401
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

do {
    let _request = try LiterLlm.embeddingRequestFromJson("{\"input\":\"Hello world\",\"model\":\"text-embedding-3-small\"}")
    _ = try await LiterLlm.embed(request: _request)
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
