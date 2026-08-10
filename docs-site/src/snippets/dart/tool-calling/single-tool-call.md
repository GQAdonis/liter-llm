---
id: fixture_dart_single_tool_call
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"messages":[{"content":"What is the weather in San Francisco?","role":"user"}],"model":"gpt-4","tool_choice":"auto","tools":[{"function":{"description":"Get the current weather for a given location","name":"get_weather","parameters":{"properties":{"location":{"description":"The city and state, e.g. San Francisco, CA","type":"string"},"unit":{"description":"The temperature unit to use","enum":["celsius","fahrenheit"],"type":"string"}},"required":["location"],"type":"object"}},"type":"function"}]}');
  final _mockUrl = _fixtureUrl("single_tool_call");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chat(req: _request);
}

```
