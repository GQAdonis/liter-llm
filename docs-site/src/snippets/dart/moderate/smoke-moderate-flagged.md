---
id: fixture_dart_smoke_moderate_flagged
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createModerationRequestFromJson(json: '{"input":"I want to hurt someone very badly","model":"omni-moderation-latest"}');
  final _mockUrl = _fixtureUrl("smoke_moderate_flagged");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.moderate(req: _request);
}

```
