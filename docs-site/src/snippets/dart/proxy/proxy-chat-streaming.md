---
id: fixture_dart_proxy_chat_streaming
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"messages":[{"content":"Count to 3","role":"user"}],"model":"openai/gpt-4o","stream":true}');
  final _mockUrl = _fixtureUrl("proxy_chat_streaming");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chatStream(req: _request).toList();
}

```
