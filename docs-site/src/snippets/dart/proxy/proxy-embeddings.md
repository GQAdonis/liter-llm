---
id: fixture_dart_proxy_embeddings
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createEmbeddingRequestFromJson(json: '{"input":"Hello world","model":"openai/text-embedding-3-small"}');
  final _mockUrl = _fixtureUrl("proxy_embeddings");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.embed(req: _request);
}

```
