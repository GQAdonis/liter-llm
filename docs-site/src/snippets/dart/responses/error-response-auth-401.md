---
id: fixture_dart_error_response_auth_401
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createCreateResponseRequestFromJson(json: '{"input":"Hello","model":"gpt-4o"}');
  final _mockUrl = _fixtureUrl("error_response_auth_401");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.createResponse(req: _request);
}

```
