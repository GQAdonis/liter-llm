---
id: fixture_dart_smoke_streaming_openai
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"max_tokens":50,"messages":[{"content":"Count from 1 to 5.","role":"user"}],"model":"openai/gpt-4o-mini"}');
  final _mockUrl = _fixtureUrl("smoke_streaming_openai");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chatStream(req: _request).toList();
}

```
