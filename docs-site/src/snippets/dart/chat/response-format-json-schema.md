---
id: fixture_dart_response_format_json_schema
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"messages":[{"content":"What is the temperature in Paris today?","role":"user"}],"model":"gpt-4","response_format":{"json_schema":{"name":"weather","schema":{"properties":{"temp":{"type":"number"}},"required":["temp"],"type":"object"}},"type":"json_schema"}}');
  final _mockUrl = _fixtureUrl("response_format_json_schema");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chat(req: _request);
}

```
