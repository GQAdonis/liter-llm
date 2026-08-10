---
id: fixture_swift_search_basic
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.searchRequestFromJson("{\"model\":\"brave/web-search\",\"query\":\"What is Rust programming language?\"}")
_ = try await LiterLlm.search(request: _request)

```
