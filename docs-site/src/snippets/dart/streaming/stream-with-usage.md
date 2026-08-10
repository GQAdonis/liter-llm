---
id: fixture_dart_stream_with_usage
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"messages":[{"content":"Say hi","role":"user"}],"model":"gpt-4","stream":true,"stream_options":{"include_usage":true}}');
  final _mockUrl = _fixtureUrl("stream_with_usage");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chatStream(req: _request).toList();
}

```
