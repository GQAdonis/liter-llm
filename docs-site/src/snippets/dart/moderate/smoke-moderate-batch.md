---
id: fixture_dart_smoke_moderate_batch
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createModerationRequestFromJson(json: '{"input":["Hello world","Nice weather today"],"model":"omni-moderation-latest"}');
  final _mockUrl = _fixtureUrl("smoke_moderate_batch");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.moderate(req: _request);
}

```
