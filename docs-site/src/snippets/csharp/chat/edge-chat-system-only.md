---
id: fixture_csharp_edge_chat_system_only
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
  "/fixtures/edge_chat_system_only"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatAsync(new ChatCompletionRequest { Messages = new List<string> { "{\"content\":\"You are a helpful and concise assistant\",\"role\":\"system\"}", "{\"content\":\"Hi\",\"role\":\"user\"}" }, Model = "gpt-4" });

```
