---
id: fixture_csharp_smoke_moderate_batch
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
  "/fixtures/smoke_moderate_batch"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ModerateAsync(new ModerationRequest { Input = JsonSerializer.Deserialize<ModerationInput>("[\"Hello world\",\"Nice weather today\"]", ConfigOptions)!, Model = "omni-moderation-latest" });

```
