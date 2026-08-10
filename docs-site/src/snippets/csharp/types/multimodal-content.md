---
id: fixture_csharp_multimodal_content
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
  "/fixtures/multimodal_content"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatAsync(new ChatCompletionRequest { MaxTokens = 100, Messages = new List<string> { "{\"content\":[{\"text\":\"What is in this image?\",\"type\":\"text\"},{\"image_url\":{\"detail\":\"low\",\"url\":\"https://upload.wikimedia.org/wikipedia/commons/thumb/4/47/PNG_transparency_demonstration_1.png/280px-PNG_transparency_demonstration_1.png\"},\"type\":\"image_url\"}],\"role\":\"user\"}" }, Model = "gpt-4o" });

```
