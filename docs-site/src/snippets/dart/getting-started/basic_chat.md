---
id: readme_dart_basic_chat
language: dart
target: dart
level: syntax
requires: []
side_effect: network
---

Send a message to any provider using the `provider/model` prefix.

```dart
import 'package:liter_llm/liter_llm.dart';
import 'dart:io';

void main() async {
  final client = await LiterLlmBridge.createClient(
    Platform.environment['OPENAI_API_KEY'] ?? '',
  );
  final request = ChatCompletionRequest(
    model: 'openai/gpt-4o',
    messages: [
      Message.user(field0: UserMessage(content: UserContent.text(field0: 'Hello!'))),
    ],
  );
  final response = await client.chat(req: request);
  print(response.choices[0].message.content);
}
```
