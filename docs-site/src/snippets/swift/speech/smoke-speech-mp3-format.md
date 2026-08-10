---
id: fixture_swift_smoke_speech_mp3_format
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.createSpeechRequestFromJson("{\"input\":\"The quick brown fox jumps over the lazy dog.\",\"model\":\"tts-1-hd\",\"response_format\":\"mp3\",\"speed\":1.0,\"voice\":\"nova\"}")
_ = try await LiterLlm.speech(request: _request)

```
