---
id: fixture_swift_edge_transcribe_with_timestamps
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.createTranscriptionRequestFromJson("{\"file\":\"audio.mp3\",\"model\":\"whisper-1\",\"response_format\":\"verbose_json\"}")
_ = try await LiterLlm.transcribe(request: _request)

```
