---
id: fixture_dart_error_moderate_bad_request
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createModerationRequestFromJson(json: '{"input":"Hello","model":"nonexistent-moderation"}');
  final _mockUrl = _fixtureUrl("error_moderate_bad_request");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.moderate(req: _request);
}

```
