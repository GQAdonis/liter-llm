---
id: fixture_swift_search_with_max_results
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.searchRequestFromJson("{\"max_results\":2,\"model\":\"brave/web-search\",\"query\":\"Rust programming\"}")
_ = try await LiterLlm.search(request: _request)

```
