---
id: fixture_csharp_anthropic_stream
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
  "/fixtures/anthropic_stream"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatStreamAsync(new ChatCompletionRequest { MaxTokens = 32, Messages = new List<string> { "{\"content\":\"Count to three, one word per response.\",\"role\":\"user\"}" }, Model = "anthropic/claude-3-5-sonnet-20241022", Stream = true });

```
