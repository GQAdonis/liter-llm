---
id: fixture_dart_edge_rerank_empty_query
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createRerankRequestFromJson(json: '{"documents":["Some document","Another document"],"model":"rerank-v3.5","query":""}');
  final _mockUrl = _fixtureUrl("edge_rerank_empty_query");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.rerank(req: _request);
}

```
