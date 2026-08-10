---
id: fixture_dart_smoke_image_with_size
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = CreateImageRequest(prompt: 'A sunset over mountains', model: 'dall-e-3', n: 1, size: '1792x1024', quality: null, style: null, responseFormat: null, user: null);
  final _mockUrl = _fixtureUrl("smoke_image_with_size");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.imageGenerate(req: _request);
}

```
