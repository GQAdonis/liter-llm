---
id: fixture_csharp_ocr_url_document
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
  "/fixtures/ocr_url_document"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.OcrAsync(new OcrRequest { Document = JsonSerializer.Deserialize<OcrDocument>("{\"type\":\"document_url\",\"url\":\"https://example.com/doc.pdf\"}", ConfigOptions)!, Model = "mistral/mistral-ocr-latest" });

```
