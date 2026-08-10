---
id: fixture_dart_developer_message
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"messages":[{"content":"You are a coding assistant. Always respond with concise code examples.","role":"developer"},{"content":"How do I reverse a string in Python?","role":"user"}],"model":"gpt-4"}');
  final _mockUrl = _fixtureUrl("developer_message");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chat(req: _request);
}

```
