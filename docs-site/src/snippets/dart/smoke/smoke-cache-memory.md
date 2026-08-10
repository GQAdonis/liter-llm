---
id: fixture_dart_smoke_cache_memory
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"max_tokens":5,"messages":[{"content":"What is 2+2? Answer with just the number.","role":"user"}],"model":"openai/gpt-4o-mini"}');
  final _mockUrl = _fixtureUrl("smoke_cache_memory");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chat(req: _request);
}

```
