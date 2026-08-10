---
id: fixture_csharp_embed_with_dimensions
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
  "/fixtures/embed_with_dimensions"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.EmbedAsync(new EmbeddingRequest { Dimensions = 256, Input = JsonSerializer.Deserialize<EmbeddingInput>("\"Hello world\"", ConfigOptions)!, Model = "text-embedding-3-small" });

```
