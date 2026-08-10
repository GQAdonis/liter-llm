---
id: fixture_dart_bedrock_chat
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"max_tokens":16,"messages":[{"content":"Say hello in one word.","role":"user"}],"model":"bedrock/anthropic.claude-3-sonnet-20240229-v1:0","temperature":0}');
  final _mockUrl = _fixtureUrl("bedrock_chat");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chat(req: _request);
}

```
