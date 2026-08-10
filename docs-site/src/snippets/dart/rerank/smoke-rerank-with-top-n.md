---
id: fixture_dart_smoke_rerank_with_top_n
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createRerankRequestFromJson(json: '{"documents":["Python is a programming language.","Cats are cute animals.","Python was created by Guido van Rossum.","The sun is a star."],"model":"rerank-v3.5","query":"What is Python?","top_n":2}');
  final _mockUrl = _fixtureUrl("smoke_rerank_with_top_n");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.rerank(req: _request);
}

```
