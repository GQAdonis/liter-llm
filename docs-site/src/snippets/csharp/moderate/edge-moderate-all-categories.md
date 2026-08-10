---
id: fixture_csharp_edge_moderate_all_categories
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
  "/fixtures/edge_moderate_all_categories"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ModerateAsync(new ModerationRequest { Input = JsonSerializer.Deserialize<ModerationInput>("\"Extremely harmful content targeting multiple categories\"", ConfigOptions)!, Model = "omni-moderation-latest" });

```
