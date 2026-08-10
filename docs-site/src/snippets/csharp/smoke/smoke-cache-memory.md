---
id: fixture_csharp_smoke_cache_memory
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
  "/fixtures/smoke_cache_memory"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatAsync(new ChatCompletionRequest { MaxTokens = 5, Messages = new List<string> { "{\"content\":\"What is 2+2? Answer with just the number.\",\"role\":\"user\"}" }, Model = "openai/gpt-4o-mini" });

```
