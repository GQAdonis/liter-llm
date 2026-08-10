---
id: fixture_swift_error_transcribe_bad_format
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

do {
    let _request = try LiterLlm.createTranscriptionRequestFromJson("{\"file\":\"audio.xyz\",\"model\":\"whisper-1\"}")
    _ = try await LiterLlm.transcribe(request: _request)
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
