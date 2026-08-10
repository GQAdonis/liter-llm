---
id: fixture_dart_stream_with_tool_calls
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"messages":[{"content":"What is the weather in NYC?","role":"user"}],"model":"gpt-4","stream":true,"tools":[{"function":{"description":"Get the current weather for a given location","name":"get_weather","parameters":{"properties":{"location":{"description":"The city and state, e.g. New York, NY","type":"string"}},"required":["location"],"type":"object"}},"type":"function"}]}');
  final _mockUrl = _fixtureUrl("stream_with_tool_calls");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chatStream(req: _request).toList();
}

```
