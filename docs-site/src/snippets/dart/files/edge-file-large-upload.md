---
id: fixture_dart_edge_file_large_upload
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'dart:io';
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createCreateFileRequestFromJson(json: '{"file":"eyJwcm9tcHQiOiAibGFyZ2UgdHJhaW5pbmcgZGF0YSJ9Cg==","filename":"large_training_data.jsonl","purpose":"fine-tune"}');
  final _mockUrl = _fixtureUrl("edge_file_large_upload");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.createFile(req: _request);
}

```
