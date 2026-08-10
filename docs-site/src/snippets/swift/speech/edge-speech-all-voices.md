---
id: fixture_swift_edge_speech_all_voices
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.createSpeechRequestFromJson("{\"input\":\"Hello world\",\"model\":\"tts-1\",\"voice\":\"nova\"}")
_ = try await LiterLlm.speech(request: _request)

```
