---
id: fixture_csharp_finish_reason_length
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
  "/fixtures/finish_reason_length"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatAsync(new ChatCompletionRequest { MaxTokens = 5, Messages = new List<string> { "{\"content\":\"Tell me a long story\",\"role\":\"user\"}" }, Model = "gpt-4" });

```
