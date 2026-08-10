---
id: fixture_csharp_batch_embed
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using System;
using System.Text.Json;
using LiterLlm;

var ConfigOptions = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
var baseUrl = (Environment.GetEnvironmentVariable("MOCK_SERVER_URL") ?? string.Empty) +
  "/fixtures/batch_embed"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.EmbedAsync(new EmbeddingRequest { Input = JsonSerializer.Deserialize<EmbeddingInput>("[\"Hello\",\"World\"]", ConfigOptions)!, Model = "text-embedding-3-small" });

```
