---
id: fixture_csharp_vertex_stream
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
  "/fixtures/vertex_stream"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatStreamAsync(new ChatCompletionRequest { MaxTokens = 32, Messages = new List<string> { "{\"content\":\"Count to three, one word per response.\",\"role\":\"user\"}" }, Model = "vertex_ai/gemini-2.0-flash", Stream = true });

```
