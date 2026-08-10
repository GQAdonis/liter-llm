---
id: fixture_dart_finish_reason_length
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"max_tokens":5,"messages":[{"content":"Tell me a long story","role":"user"}],"model":"gpt-4"}');
  final _mockUrl = _fixtureUrl("finish_reason_length");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chat(req: _request);
}

```
