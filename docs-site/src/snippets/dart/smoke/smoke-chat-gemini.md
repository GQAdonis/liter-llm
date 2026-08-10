---
id: fixture_dart_smoke_chat_gemini
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"max_tokens":10,"messages":[{"content":"Say hello in exactly one word.","role":"user"}],"model":"gemini/gemini-2.5-flash-lite"}');
  final _mockUrl = _fixtureUrl("smoke_chat_gemini");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chat(req: _request);
}

```
