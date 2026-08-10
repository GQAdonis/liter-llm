---
id: fixture_swift_contract_search
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.searchRequestFromJson("{\"model\":\"brave/web-search\",\"query\":\"contract test query\"}")
_ = try await LiterLlm.search(request: _request)

```
