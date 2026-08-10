---
id: fixture_dart_edge_stream_function_call
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"messages":[{"content":"What\'s the weather?","role":"user"}],"model":"gpt-4","tools":[{"function":{"name":"get_weather","parameters":{"properties":{"city":{"type":"string"}},"type":"object"}},"type":"function"}]}');
  final _mockUrl = _fixtureUrl("edge_stream_function_call");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chatStream(req: _request).toList();
}

```
