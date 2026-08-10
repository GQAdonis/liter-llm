---
id: fixture_dart_proxy_upstream_429
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"messages":[{"content":"Hello","role":"user"}],"model":"openai/gpt-4o"}');
  final _mockUrl = _fixtureUrl("proxy_upstream_429");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chat(req: _request);
}

```
