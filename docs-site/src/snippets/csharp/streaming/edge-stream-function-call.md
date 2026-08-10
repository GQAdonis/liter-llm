---
id: fixture_csharp_edge_stream_function_call
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
  "/fixtures/edge_stream_function_call"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatStreamAsync(new ChatCompletionRequest { Messages = new List<string> { "{\"content\":\"What's the weather?\",\"role\":\"user\"}" }, Model = "gpt-4", Tools = new List<string> { "{\"function\":{\"name\":\"get_weather\",\"parameters\":{\"properties\":{\"city\":{\"type\":\"string\"}},\"type\":\"object\"}},\"type\":\"function\"}" } });

```
