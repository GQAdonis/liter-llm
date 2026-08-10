---
id: fixture_csharp_response_format_json_object
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
  "/fixtures/response_format_json_object"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatAsync(new ChatCompletionRequest { Messages = new List<string> { "{\"content\":\"Respond with JSON only.\",\"role\":\"system\"}", "{\"content\":\"Give me a user object with name and age fields.\",\"role\":\"user\"}" }, Model = "gpt-4", ResponseFormat = JsonSerializer.Deserialize<ResponseFormat>("{\"type\":\"json_object\"}", ConfigOptions)! });

```
