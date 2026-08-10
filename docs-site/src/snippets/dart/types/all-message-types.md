---
id: fixture_dart_all_message_types
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"messages":[{"content":"You are a helpful assistant.","role":"system"},{"content":"What is the weather in Paris?","role":"user"},{"content":null,"role":"assistant","tool_calls":[{"function":{"arguments":"{\\"location\\": \\"Paris, France\\"}","name":"get_weather"},"id":"call_xyz789","type":"function"}]},{"content":"{\\"temperature\\": 18, \\"unit\\": \\"celsius\\", \\"description\\": \\"Partly cloudy\\"}","role":"tool","tool_call_id":"call_xyz789"}],"model":"gpt-4"}');
  final _mockUrl = _fixtureUrl("all_message_types");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chat(req: _request);
}

```
