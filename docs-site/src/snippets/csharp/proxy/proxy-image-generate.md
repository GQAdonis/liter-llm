---
id: fixture_csharp_proxy_image_generate
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
  "/fixtures/proxy_image_generate"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ImageGenerateAsync(new CreateImageRequest { Model = "dall-e-3", N = 1, Prompt = "A sunset over the ocean", Size = "1024x1024" });

```
