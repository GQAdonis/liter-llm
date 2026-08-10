---
id: fixture_dart_smoke_create_response
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createCreateResponseRequestFromJson(json: '{"input":"Explain quantum computing in one sentence.","model":"gpt-4o"}');
  final _mockUrl = _fixtureUrl("smoke_create_response");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.createResponse(req: _request);
}

```
