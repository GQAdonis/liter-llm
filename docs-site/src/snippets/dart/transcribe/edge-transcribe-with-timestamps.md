---
id: fixture_dart_edge_transcribe_with_timestamps
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = CreateTranscriptionRequest(model: 'whisper-1', file: 'audio.mp3', language: null, prompt: null, responseFormat: 'verbose_json', temperature: null);
  final _mockUrl = _fixtureUrl("edge_transcribe_with_timestamps");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.transcribe(req: _request);
}

```
