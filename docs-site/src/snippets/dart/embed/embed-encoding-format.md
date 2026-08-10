---
id: fixture_dart_embed_encoding_format
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createEmbeddingRequestFromJson(json: '{"encoding_format":"float","input":"Test input","model":"text-embedding-3-small"}');
  final _mockUrl = _fixtureUrl("embed_encoding_format");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.embed(req: _request);
}

```
