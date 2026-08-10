---
id: fixture_csharp_smoke_create_file
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
  "/fixtures/smoke_create_file"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.CreateFileAsync(new CreateFileRequest { File = "eyJwcm9tcHQiOiAiaGVsbG8ifQo=", Filename = "training_data.jsonl", Purpose = JsonSerializer.Deserialize<FilePurpose>("\"fine-tune\"", ConfigOptions)! });

```
