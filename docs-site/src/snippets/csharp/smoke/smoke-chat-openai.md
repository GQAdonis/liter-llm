---
id: fixture_csharp_smoke_chat_openai
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
  "/fixtures/smoke_chat_openai"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatAsync(new ChatCompletionRequest { MaxTokens = 10, Messages = new List<string> { "{\"content\":\"Say hello in exactly one word.\",\"role\":\"user\"}" }, Model = "openai/gpt-4o-mini" });

```
