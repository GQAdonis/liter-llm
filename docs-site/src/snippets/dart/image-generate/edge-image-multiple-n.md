---
id: fixture_dart_edge_image_multiple_n
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = CreateImageRequest(prompt: 'A cat', model: 'dall-e-3', n: 3, size: null, quality: null, style: null, responseFormat: null, user: null);
  final _mockUrl = _fixtureUrl("edge_image_multiple_n");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.imageGenerate(req: _request);
}

```
