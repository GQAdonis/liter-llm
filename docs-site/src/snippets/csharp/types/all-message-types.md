---
id: fixture_csharp_all_message_types
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
  "/fixtures/all_message_types"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatAsync(new ChatCompletionRequest { Messages = new List<string> { "{\"content\":\"You are a helpful assistant.\",\"role\":\"system\"}", "{\"content\":\"What is the weather in Paris?\",\"role\":\"user\"}", "{\"content\":null,\"role\":\"assistant\",\"tool_calls\":[{\"function\":{\"arguments\":\"{\\\"location\\\": \\\"Paris, France\\\"}\",\"name\":\"get_weather\"},\"id\":\"call_xyz789\",\"type\":\"function\"}]}", "{\"content\":\"{\\\"temperature\\\": 18, \\\"unit\\\": \\\"celsius\\\", \\\"description\\\": \\\"Partly cloudy\\\"}\",\"role\":\"tool\",\"tool_call_id\":\"call_xyz789\"}" }, Model = "gpt-4" });

```
