---
id: fixture_dart_edge_chat_system_only
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"messages":[{"content":"You are a helpful and concise assistant","role":"system"},{"content":"Hi","role":"user"}],"model":"gpt-4"}');
  final _mockUrl = _fixtureUrl("edge_chat_system_only");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chat(req: _request);
}

```
