---
id: fixture_swift_edge_transcribe_empty_audio
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.createTranscriptionRequestFromJson("{\"file\":\"silence.mp3\",\"model\":\"whisper-1\"}")
_ = try await LiterLlm.transcribe(request: _request)

```
