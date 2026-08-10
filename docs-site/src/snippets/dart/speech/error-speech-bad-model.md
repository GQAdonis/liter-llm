---
id: fixture_dart_error_speech_bad_model
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = CreateSpeechRequest(model: 'tts-nonexistent', input: 'Hello', voice: 'alloy', responseFormat: null, speed: null);
  final _mockUrl = _fixtureUrl("error_speech_bad_model");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.speech(req: _request);
}

```
