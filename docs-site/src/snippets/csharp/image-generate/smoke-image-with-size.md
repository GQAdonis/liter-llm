---
id: fixture_csharp_smoke_image_with_size
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
  "/fixtures/smoke_image_with_size"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ImageGenerateAsync(new CreateImageRequest { Model = "dall-e-3", N = 1, Prompt = "A sunset over mountains", Size = "1792x1024" });

```
