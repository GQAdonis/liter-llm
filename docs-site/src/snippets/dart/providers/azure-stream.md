---
id: fixture_dart_azure_stream
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"messages":[{"content":"Count to 3","role":"user"}],"model":"azure/gpt-4","stream":true,"temperature":0}');
  final _mockUrl = _fixtureUrl("azure_stream");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chatStream(req: _request).toList();
}

```
