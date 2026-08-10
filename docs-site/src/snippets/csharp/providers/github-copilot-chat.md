---
id: fixture_csharp_github_copilot_chat
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
  "/fixtures/github_copilot_chat"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatAsync(new ChatCompletionRequest { MaxTokens = 16, Messages = new List<string> { "{\"content\":\"Say hello in one word.\",\"role\":\"user\"}" }, Model = "github_copilot/gpt-4o", Temperature = 0 });

```
