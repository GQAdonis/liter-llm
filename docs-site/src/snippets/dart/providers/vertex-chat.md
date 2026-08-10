---
id: fixture_dart_vertex_chat
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"max_tokens":16,"messages":[{"content":"Say hello in one word.","role":"user"}],"model":"vertex_ai/gemini-2.0-flash","temperature":0}');
  final _mockUrl = _fixtureUrl("vertex_chat");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chat(req: _request);
}

```
