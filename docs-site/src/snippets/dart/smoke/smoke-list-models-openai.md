---
id: fixture_dart_smoke_list_models_openai
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _mockUrl = _fixtureUrl("smoke_list_models_openai");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.listModels();
}

```
