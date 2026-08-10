---
id: fixture_dart_smoke_rerank_return_docs
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createRerankRequestFromJson(json: '{"documents":["Rust is a systems programming language.","Iron rusts when exposed to water."],"model":"rerank-v3.5","query":"What is Rust?","return_documents":true}');
  final _mockUrl = _fixtureUrl("smoke_rerank_return_docs");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.rerank(req: _request);
}

```
