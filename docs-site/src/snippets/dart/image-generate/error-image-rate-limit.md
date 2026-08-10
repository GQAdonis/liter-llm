---
id: fixture_dart_error_image_rate_limit
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = CreateImageRequest(prompt: 'A cat', model: 'dall-e-3', n: 1, size: '1024x1024', quality: null, style: null, responseFormat: null, user: null);
  final _mockUrl = _fixtureUrl("error_image_rate_limit");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.imageGenerate(req: _request);
}

```
