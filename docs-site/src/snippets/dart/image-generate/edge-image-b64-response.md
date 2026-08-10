---
id: fixture_dart_edge_image_b64_response
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = CreateImageRequest(prompt: 'A blue circle', model: 'dall-e-3', n: 1, size: '1024x1024', quality: null, style: null, responseFormat: 'b64_json', user: null);
  final _mockUrl = _fixtureUrl("edge_image_b64_response");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.imageGenerate(req: _request);
}

```
