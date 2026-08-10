---
id: fixture_csharp_anthropic_chat
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
  "/fixtures/anthropic_chat"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatAsync(new ChatCompletionRequest { MaxTokens = 16, Messages = new List<string> { "{\"content\":\"You are a helpful assistant.\",\"role\":\"system\"}", "{\"content\":\"Say hello in one word.\",\"role\":\"user\"}" }, Model = "anthropic/claude-3-5-sonnet-20241022", Temperature = 0 });

```
