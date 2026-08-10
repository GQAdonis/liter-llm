---
id: fixture_csharp_seed_parameter
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: network
---

```csharp title="C#"
using System;
using LiterLlm;

var baseUrl = (Environment.GetEnvironmentVariable("MOCK_SERVER_URL") ?? string.Empty) +
  "/fixtures/seed_parameter"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatAsync(new ChatCompletionRequest { Messages = new List<string> { "{\"content\":\"Pick a random number\",\"role\":\"user\"}" }, Model = "gpt-4", Seed = 42 });

```
