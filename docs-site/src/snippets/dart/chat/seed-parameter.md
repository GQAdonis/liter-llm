---
id: fixture_dart_seed_parameter
language: dart
target: dart
level: typecheck
requires: []
side_effect: network
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"messages":[{"content":"Pick a random number","role":"user"}],"model":"gpt-4","seed":42}');
  final _mockUrl = _fixtureUrl("seed_parameter");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chat(req: _request);
}

```
