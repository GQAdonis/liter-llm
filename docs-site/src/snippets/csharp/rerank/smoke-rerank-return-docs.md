---
id: fixture_csharp_smoke_rerank_return_docs
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
  "/fixtures/smoke_rerank_return_docs"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.RerankAsync(new RerankRequest { Documents = new List<string> { "Rust is a systems programming language.", "Iron rusts when exposed to water." }, Model = "rerank-v3.5", Query = "What is Rust?", ReturnDocuments = true });

```
