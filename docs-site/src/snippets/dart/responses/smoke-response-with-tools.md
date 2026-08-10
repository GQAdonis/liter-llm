---
id: fixture_dart_smoke_response_with_tools
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createCreateResponseRequestFromJson(json: '{"input":"What is the weather in San Francisco?","model":"gpt-4o","tools":[{"description":"Get current weather for a location","name":"get_weather","parameters":{"properties":{"location":{"type":"string"}},"required":["location"],"type":"object"},"type":"function"}]}');
  final _mockUrl = _fixtureUrl("smoke_response_with_tools");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.createResponse(req: _request);
}

```
