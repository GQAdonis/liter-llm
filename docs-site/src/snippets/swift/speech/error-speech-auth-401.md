---
id: fixture_swift_error_speech_auth_401
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

do {
    let _request = try LiterLlm.createSpeechRequestFromJson("{\"input\":\"Hello\",\"model\":\"tts-1\",\"voice\":\"alloy\"}")
    _ = try await LiterLlm.speech(request: _request)
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
