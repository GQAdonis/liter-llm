---
id: fixture_csharp_tool_choice_specific
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
  "/fixtures/tool_choice_specific"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatAsync(new ChatCompletionRequest { Messages = new List<string> { "{\"content\":\"What is the weather in Paris?\",\"role\":\"user\"}" }, Model = "gpt-4", ToolChoice = JsonSerializer.Deserialize<ToolChoice>("{\"function\":{\"name\":\"get_weather\"},\"type\":\"function\"}", ConfigOptions)!, Tools = new List<string> { "{\"function\":{\"description\":\"Get the current weather for a given location\",\"name\":\"get_weather\",\"parameters\":{\"properties\":{\"location\":{\"description\":\"The city name\",\"type\":\"string\"}},\"required\":[\"location\"],\"type\":\"object\"}},\"type\":\"function\"}", "{\"function\":{\"description\":\"Search the web for information\",\"name\":\"search_web\",\"parameters\":{\"properties\":{\"query\":{\"description\":\"The search query\",\"type\":\"string\"}},\"required\":[\"query\"],\"type\":\"object\"}},\"type\":\"function\"}" } });

```
