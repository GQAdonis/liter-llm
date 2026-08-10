---
id: fixture_csharp_multi_turn_conversation
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using System;
using LiterLlm;

var baseUrl = (Environment.GetEnvironmentVariable("MOCK_SERVER_URL") ?? string.Empty) +
  "/fixtures/multi_turn_conversation"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatAsync(new ChatCompletionRequest { Messages = new List<string> { "{\"content\":\"You are a helpful assistant.\",\"role\":\"system\"}", "{\"content\":\"What is 2 + 2?\",\"role\":\"user\"}", "{\"content\":\"2 + 2 equals 4.\",\"role\":\"assistant\"}", "{\"content\":\"And what is 4 + 4?\",\"role\":\"user\"}" }, Model = "gpt-4" });

```
