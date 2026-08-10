---
id: fixture_csharp_edge_response_empty_output
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
  "/fixtures/edge_response_empty_output"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.CreateResponseAsync(new CreateResponseRequest { Input = "", Model = "gpt-4o" });

```
