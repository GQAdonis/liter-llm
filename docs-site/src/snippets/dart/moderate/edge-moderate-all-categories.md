---
id: fixture_dart_edge_moderate_all_categories
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createModerationRequestFromJson(json: '{"input":"Extremely harmful content targeting multiple categories","model":"omni-moderation-latest"}');
  final _mockUrl = _fixtureUrl("edge_moderate_all_categories");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.moderate(req: _request);
}

```
