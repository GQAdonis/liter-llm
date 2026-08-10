---
id: fixture_dart_tool_choice_required
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"messages":[{"content":"What is the weather today?","role":"user"}],"model":"gpt-4","tool_choice":"required","tools":[{"function":{"description":"Get the current weather for a given location","name":"get_weather","parameters":{"properties":{"location":{"description":"The city name","type":"string"}},"required":["location"],"type":"object"}},"type":"function"}]}');
  final _mockUrl = _fixtureUrl("tool_choice_required");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chat(req: _request);
}

```
