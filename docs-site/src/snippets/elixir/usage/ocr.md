---
id: legacy_elixir_usage_ocr
language: elixir
target: elixir
level: syntax
requires: []
side_effect: network
---

<!-- snippet:compile-only -->

```elixir
{:ok, client} = LiterLlm.create_client(System.get_env("MISTRAL_API_KEY"))

request =
  Jason.encode!(%{
    model: "mistral/mistral-ocr-latest",
    document: %{type: "document_url", url: "https://example.com/invoice.pdf"}
  })

{:ok, result} = LiterLlm.defaultclient_ocr_async(client, request)

for page <- result.pages do
  IO.puts("Page #{page.index}: #{String.slice(page.markdown, 0, 100)}...")
end
```
