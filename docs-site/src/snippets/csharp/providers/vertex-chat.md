---
id: fixture_csharp_vertex_chat
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
  "/fixtures/vertex_chat"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatAsync(new ChatCompletionRequest { MaxTokens = 16, Messages = new List<string> { "{\"content\":\"Say hello in one word.\",\"role\":\"user\"}" }, Model = "vertex_ai/gemini-2.0-flash", Temperature = 0 });

```
