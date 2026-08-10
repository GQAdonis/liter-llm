---
id: fixture_swift_local_provider_vllm
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.chatCompletionRequestFromJson("{\"messages\":[{\"content\":\"Hello\",\"role\":\"user\"}],\"model\":\"vllm/meta-llama/Llama-3.2-1B\"}")
_ = try await LiterLlm.chat(request: _request)

```
