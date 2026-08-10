---
id: fixture_dart_multi_turn_conversation
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"messages":[{"content":"You are a helpful assistant.","role":"system"},{"content":"What is 2 + 2?","role":"user"},{"content":"2 + 2 equals 4.","role":"assistant"},{"content":"And what is 4 + 4?","role":"user"}],"model":"gpt-4"}');
  final _mockUrl = _fixtureUrl("multi_turn_conversation");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chat(req: _request);
}

```
