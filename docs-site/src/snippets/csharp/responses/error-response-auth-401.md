---
id: fixture_csharp_error_response_auth_401
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
  "/fixtures/error_response_auth_401"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.CreateResponseAsync(new CreateResponseRequest { Input = "Hello", Model = "gpt-4o" });

```
