---
id: fixture_csharp_edge_image_b64_response
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
  "/fixtures/edge_image_b64_response"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ImageGenerateAsync(new CreateImageRequest { Model = "dall-e-3", N = 1, Prompt = "A blue circle", ResponseFormat = "b64_json", Size = "1024x1024" });

```
