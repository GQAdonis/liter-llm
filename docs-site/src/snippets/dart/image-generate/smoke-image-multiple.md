---
id: fixture_dart_smoke_image_multiple
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = CreateImageRequest(prompt: 'A red bicycle', model: 'dall-e-2', n: 3, size: '256x256', quality: null, style: null, responseFormat: null, user: null);
  final _mockUrl = _fixtureUrl("smoke_image_multiple");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.imageGenerate(req: _request);
}

```
