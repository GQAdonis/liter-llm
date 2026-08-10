---
id: fixture_dart_smoke_rerank_basic
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createRerankRequestFromJson(json: '{"documents":["Machine learning is a subset of AI.","The weather is sunny today.","Deep learning uses neural networks."],"model":"rerank-v3.5","query":"What is machine learning?"}');
  final _mockUrl = _fixtureUrl("smoke_rerank_basic");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.rerank(req: _request);
}

```
