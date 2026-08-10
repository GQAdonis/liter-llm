---
id: fixture_swift_seed_parameter
language: swift
target: swift
level: typecheck
requires: []
side_effect: network
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"messages\":[{\"content\":\"Pick a random number\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"seed\":42}")
_ = try await LiterLlm.chat(request: _request)

```
