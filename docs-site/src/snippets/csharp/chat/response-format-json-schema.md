---
id: fixture_csharp_response_format_json_schema
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
  "/fixtures/response_format_json_schema"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatAsync(new ChatCompletionRequest { Messages = new List<string> { "{\"content\":\"What is the temperature in Paris today?\",\"role\":\"user\"}" }, Model = "gpt-4", ResponseFormat = JsonSerializer.Deserialize<ResponseFormat>("{\"json_schema\":{\"name\":\"weather\",\"schema\":{\"properties\":{\"temp\":{\"type\":\"number\"}},\"required\":[\"temp\"],\"type\":\"object\"}},\"type\":\"json_schema\"}", ConfigOptions)! });

```
