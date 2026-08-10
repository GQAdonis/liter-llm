---
id: fixture_csharp_error_file_bad_purpose
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
  "/fixtures/error_file_bad_purpose"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.CreateFileAsync(new CreateFileRequest { File = "data.jsonl", Purpose = JsonSerializer.Deserialize<FilePurpose>("\"invalid-purpose\"", ConfigOptions)! });

```
