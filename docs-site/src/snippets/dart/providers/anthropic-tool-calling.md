---
id: fixture_dart_anthropic_tool_calling
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"max_tokens":256,"messages":[{"content":"What is the weather in London?","role":"user"}],"model":"anthropic/claude-3-5-sonnet-20241022","tool_choice":"auto","tools":[{"function":{"description":"Get the current weather for a given location","name":"get_weather","parameters":{"properties":{"location":{"description":"The city and country, e.g. London, UK","type":"string"},"unit":{"description":"The temperature unit to use","enum":["celsius","fahrenheit"],"type":"string"}},"required":["location"],"type":"object"}},"type":"function"}]}');
  final _mockUrl = _fixtureUrl("anthropic_tool_calling");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chat(req: _request);
}

```
