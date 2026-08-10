---
id: fixture_dart_smoke_image_basic
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = CreateImageRequest(prompt: 'A white cat sitting on a windowsill', model: 'dall-e-3', n: 1, size: '1024x1024', quality: null, style: null, responseFormat: null, user: null);
  final _mockUrl = _fixtureUrl("smoke_image_basic");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.imageGenerate(req: _request);
}

```
