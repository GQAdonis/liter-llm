---
id: fixture_dart_edge_rerank_single_doc
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createRerankRequestFromJson(json: '{"documents":["Artificial intelligence is the simulation of human intelligence."],"model":"rerank-v3.5","query":"What is AI?"}');
  final _mockUrl = _fixtureUrl("edge_rerank_single_doc");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.rerank(req: _request);
}

```
