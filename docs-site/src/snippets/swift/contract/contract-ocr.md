---
id: fixture_swift_contract_ocr
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import LiterLlm

let _request = try LiterLlm.ocrRequestFromJson("{\"document\":{\"type\":\"document_url\",\"url\":\"https://example.com/contract-test.pdf\"},\"model\":\"mistral/mistral-ocr-latest\"}")
_ = try await LiterLlm.ocr(request: _request)

```
