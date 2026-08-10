---
id: fixture_csharp_developer_message
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
  "/fixtures/developer_message"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatAsync(new ChatCompletionRequest { Messages = new List<string> { "{\"content\":\"You are a coding assistant. Always respond with concise code examples.\",\"role\":\"developer\"}", "{\"content\":\"How do I reverse a string in Python?\",\"role\":\"user\"}" }, Model = "gpt-4" });

```
