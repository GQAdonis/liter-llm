---
id: fixture_csharp_edge_embed_batch_input
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
  "/fixtures/edge_embed_batch_input"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.EmbedAsync(new EmbeddingRequest { Input = JsonSerializer.Deserialize<EmbeddingInput>("[\"Hello world\",\"Goodbye world\"]", ConfigOptions)!, Model = "text-embedding-3-small" });

```
