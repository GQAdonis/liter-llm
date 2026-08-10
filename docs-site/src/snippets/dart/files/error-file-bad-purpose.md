---
id: fixture_dart_error_file_bad_purpose
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
  final _request = await createCreateFileRequestFromJson(json: '{"file":"data.jsonl","purpose":"invalid-purpose"}');
  final _mockUrl = _fixtureUrl("error_file_bad_purpose");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.createFile(req: _request);
}

```
