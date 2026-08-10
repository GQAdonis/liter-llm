---
id: fixture_csharp_smoke_create_response
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
  "/fixtures/smoke_create_response"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.CreateResponseAsync(new CreateResponseRequest { Input = "Explain quantum computing in one sentence.", Model = "gpt-4o" });

```
