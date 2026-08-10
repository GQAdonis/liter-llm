---
id: fixture_dart_local_chat_ollama
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"max_tokens":10,"messages":[{"content":"Say hello in one word.","role":"user"}],"model":"ollama/qwen2:0.5b"}');
  final _mockUrl = _fixtureUrl("local_chat_ollama");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chat(req: _request);
}

```
