---
id: fixture_dart_multimodal_content
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = await createChatCompletionRequestFromJson(json: '{"max_tokens":100,"messages":[{"content":[{"text":"What is in this image?","type":"text"},{"image_url":{"detail":"low","url":"https://upload.wikimedia.org/wikipedia/commons/thumb/4/47/PNG_transparency_demonstration_1.png/280px-PNG_transparency_demonstration_1.png"},"type":"image_url"}],"role":"user"}],"model":"gpt-4o"}');
  final _mockUrl = _fixtureUrl("multimodal_content");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.chat(req: _request);
}

```
