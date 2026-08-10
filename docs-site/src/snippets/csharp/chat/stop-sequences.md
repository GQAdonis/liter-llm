---
id: fixture_csharp_stop_sequences
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
  "/fixtures/stop_sequences"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatAsync(new ChatCompletionRequest { Messages = new List<string> { "{\"content\":\"List items until you see STOP\",\"role\":\"user\"}" }, Model = "gpt-4", Stop = JsonSerializer.Deserialize<StopSequence>("[\"STOP\",\"END\"]", ConfigOptions)! });

```
