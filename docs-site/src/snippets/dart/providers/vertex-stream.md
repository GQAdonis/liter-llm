---
id: fixture_dart_vertex_stream
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"max_tokens":32,"messages":[{"content":"Count to three, one word per response.","role":"user"}],"model":"vertex_ai/gemini-2.0-flash","stream":true}');
  final _mockUrl = _fixtureUrl("vertex_stream");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chatStream(req: _request).toList();
}

```
