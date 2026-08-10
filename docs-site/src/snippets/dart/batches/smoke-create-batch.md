---
id: fixture_dart_smoke_create_batch
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = CreateBatchRequest(inputFileId: 'file-abc123', endpoint: '/v1/chat/completions', completionWindow: '24h', metadata: null);
  final _mockUrl = _fixtureUrl("smoke_create_batch");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.createBatch(req: _request);
}

```
