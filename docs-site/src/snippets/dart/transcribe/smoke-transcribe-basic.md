---
id: fixture_dart_smoke_transcribe_basic
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = CreateTranscriptionRequest(model: 'whisper-1', file: 'audio.mp3', language: null, prompt: null, responseFormat: null, temperature: null);
  final _mockUrl = _fixtureUrl("smoke_transcribe_basic");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.transcribe(req: _request);
}

```
