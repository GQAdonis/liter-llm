---
id: fixture_dart_vertex_embed
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createEmbeddingRequestFromJson(json: '{"input":"Hello","model":"vertex_ai/text-embedding-005"}');
  final _mockUrl = _fixtureUrl("vertex_embed");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.embed(req: _request);
}

```
