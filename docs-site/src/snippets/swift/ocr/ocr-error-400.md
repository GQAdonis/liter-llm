---
id: fixture_swift_ocr_error_400
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

do {
    let _request = try LiterLlm.ocrRequestFromJson("{\"document\":{\"type\":\"document_url\",\"url\":\"invalid://url\"},\"model\":\"mistral/mistral-ocr-latest\"}")
    _ = try await LiterLlm.ocr(request: _request)
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
