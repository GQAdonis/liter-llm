---
id: fixture_dart_proxy_rerank
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createRerankRequestFromJson(json: '{"documents":["Deep learning is a subset of machine learning using neural networks.","The stock market closed higher today."],"model":"rerank-v3.5","query":"What is deep learning?"}');
  final _mockUrl = _fixtureUrl("proxy_rerank");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.rerank(req: _request);
}

```
