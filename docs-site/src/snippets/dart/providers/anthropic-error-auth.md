---
id: fixture_dart_anthropic_error_auth
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"messages":[{"content":"Hello","role":"user"}],"model":"anthropic/claude-3-5-sonnet-20241022"}');
  final _mockUrl = _fixtureUrl("anthropic_error_auth");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chat(req: _request);
}

```
