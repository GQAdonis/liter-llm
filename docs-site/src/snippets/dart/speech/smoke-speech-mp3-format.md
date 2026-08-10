---
id: fixture_dart_smoke_speech_mp3_format
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = CreateSpeechRequest(model: 'tts-1-hd', input: 'The quick brown fox jumps over the lazy dog.', voice: 'nova', responseFormat: 'mp3', speed: 1.0);
  final _mockUrl = _fixtureUrl("smoke_speech_mp3_format");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.speech(req: _request);
}

```
