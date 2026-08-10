---
id: fixture_swift_smoke_speech_basic
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.createSpeechRequestFromJson("{\"input\":\"Hello, world!\",\"model\":\"tts-1\",\"voice\":\"alloy\"}")
_ = try await LiterLlm.speech(request: _request)

```
