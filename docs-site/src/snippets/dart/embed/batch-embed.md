---
id: fixture_dart_batch_embed
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createEmbeddingRequestFromJson(json: '{"input":["Hello","World"],"model":"text-embedding-3-small"}');
  final _mockUrl = _fixtureUrl("batch_embed");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.embed(req: _request);
}

```
