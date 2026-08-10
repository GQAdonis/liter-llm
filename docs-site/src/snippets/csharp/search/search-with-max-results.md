---
id: fixture_csharp_search_with_max_results
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
  "/fixtures/search_with_max_results"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.SearchAsync(new SearchRequest { MaxResults = 2, Model = "brave/web-search", Query = "Rust programming" });

```
