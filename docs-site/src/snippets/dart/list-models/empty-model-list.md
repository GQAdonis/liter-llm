---
id: fixture_dart_empty_model_list
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _mockUrl = _fixtureUrl("empty_model_list");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.listModels();
}

```
