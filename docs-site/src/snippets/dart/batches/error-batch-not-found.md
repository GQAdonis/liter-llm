---
id: fixture_dart_error_batch_not_found
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _mockUrl = _fixtureUrl("error_batch_not_found");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.retrieveBatch(batchId: 'batch-nonexistent');
}

```
