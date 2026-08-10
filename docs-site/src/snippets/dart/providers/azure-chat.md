---
id: fixture_dart_azure_chat
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"max_tokens":16,"messages":[{"content":"Say hello","role":"user"}],"model":"azure/gpt-4","temperature":0}');
  final _mockUrl = _fixtureUrl("azure_chat");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chat(req: _request);
}

```
