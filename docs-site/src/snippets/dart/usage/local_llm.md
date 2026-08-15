---
id: legacy_dart_usage_local_llm
language: dart
target: dart
level: syntax
requires: []
side_effect: network
---

<!-- snippet:compile-only -->

```dart
import 'package:liter_llm/liter_llm.dart';

void main() async {
  final client = await LiterLlmBridge.createClient(
    '',
    baseUrl: 'http://localhost:11434/v1',
  );
  final request = ChatCompletionRequest(
    model: 'ollama/qwen2:0.5b',
    messages: [
      Message.user(field0: UserMessage(content: UserContent.text(field0: 'Hello!'))),
    ],
  );
  final response = await client.chat(req: request);
  print(response.choices[0].message.content);
}
```
