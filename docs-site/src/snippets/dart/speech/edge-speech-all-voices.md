---
id: fixture_dart_edge_speech_all_voices
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = CreateSpeechRequest(model: 'tts-1', input: 'Hello world', voice: 'nova', responseFormat: null, speed: null);
  final _mockUrl = _fixtureUrl("edge_speech_all_voices");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.speech(req: _request);
}

```
