---
id: fixture_swift_search_empty_results
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.searchRequestFromJson("{\"model\":\"brave/web-search\",\"query\":\"xyznonexistent12345xyz\"}")
_ = try await LiterLlm.search(request: _request)

```
