---
id: fixture_dart_response_format_json_object
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"messages":[{"content":"Respond with JSON only.","role":"system"},{"content":"Give me a user object with name and age fields.","role":"user"}],"model":"gpt-4","response_format":{"type":"json_object"}}');
  final _mockUrl = _fixtureUrl("response_format_json_object");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chat(req: _request);
}

```
