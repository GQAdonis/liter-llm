---
id: fixture_dart_tool_choice_specific
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"messages":[{"content":"What is the weather in Paris?","role":"user"}],"model":"gpt-4","tool_choice":{"function":{"name":"get_weather"},"type":"function"},"tools":[{"function":{"description":"Get the current weather for a given location","name":"get_weather","parameters":{"properties":{"location":{"description":"The city name","type":"string"}},"required":["location"],"type":"object"}},"type":"function"},{"function":{"description":"Search the web for information","name":"search_web","parameters":{"properties":{"query":{"description":"The search query","type":"string"}},"required":["query"],"type":"object"}},"type":"function"}]}');
  final _mockUrl = _fixtureUrl("tool_choice_specific");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chat(req: _request);
}

```
