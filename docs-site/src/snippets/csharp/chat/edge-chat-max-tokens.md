---
id: fixture_csharp_edge_chat_max_tokens
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
  "/fixtures/edge_chat_max_tokens"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatAsync(new ChatCompletionRequest { MaxTokens = 1, Messages = new List<string> { "{\"content\":\"Write a story\",\"role\":\"user\"}" }, Model = "gpt-4" });

```
