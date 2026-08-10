---
id: fixture_dart_ocr_url_document
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createOcrRequestFromJson(json: '{"document":{"type":"document_url","url":"https://example.com/doc.pdf"},"model":"mistral/mistral-ocr-latest"}');
  final _mockUrl = _fixtureUrl("ocr_url_document");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.ocr(req: _request);
}

```
