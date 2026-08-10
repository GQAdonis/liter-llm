---
id: fixture_dart_smoke_create_file
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
  final _request = await createCreateFileRequestFromJson(json: '{"file":"eyJwcm9tcHQiOiAiaGVsbG8ifQo=","filename":"training_data.jsonl","purpose":"fine-tune"}');
  final _mockUrl = _fixtureUrl("smoke_create_file");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.createFile(req: _request);
}

```
