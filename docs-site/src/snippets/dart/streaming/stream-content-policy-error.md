---
id: fixture_dart_stream_content_policy_error
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"messages":[{"content":"Generate harmful content","role":"user"}],"model":"gpt-4o","stream":true}');
  final _mockUrl = _fixtureUrl("stream_content_policy_error");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chatStream(req: _request).toList();
}

```
