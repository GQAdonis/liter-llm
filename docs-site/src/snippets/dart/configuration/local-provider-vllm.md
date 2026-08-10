---
id: fixture_dart_local_provider_vllm
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"messages":[{"content":"Hello","role":"user"}],"model":"vllm/meta-llama/Llama-3.2-1B"}');
  final _mockUrl = _fixtureUrl("local_provider_vllm");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chat(req: _request);
}

```
