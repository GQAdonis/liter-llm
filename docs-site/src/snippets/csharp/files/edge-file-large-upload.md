---
id: fixture_csharp_edge_file_large_upload
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
  "/fixtures/edge_file_large_upload"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.CreateFileAsync(new CreateFileRequest { File = "eyJwcm9tcHQiOiAibGFyZ2UgdHJhaW5pbmcgZGF0YSJ9Cg==", Filename = "large_training_data.jsonl", Purpose = JsonSerializer.Deserialize<FilePurpose>("\"fine-tune\"", ConfigOptions)! });

```
