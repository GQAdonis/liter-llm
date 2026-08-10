---
id: fixture_swift_search_error_401
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

do {
    let _request = try LiterLlm.searchRequestFromJson("{\"model\":\"brave/web-search\",\"query\":\"test\"}")
    _ = try await LiterLlm.search(request: _request)
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
