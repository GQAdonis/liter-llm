---
id: fixture_csharp_basic_chat
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
  "/fixtures/basic_chat"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatAsync(new ChatCompletionRequest { Messages = new List<string> { "{\"content\":\"Say hello\",\"role\":\"user\"}" }, Model = "gpt-4", Temperature = 0 });

```
