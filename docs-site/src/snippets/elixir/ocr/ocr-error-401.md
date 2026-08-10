---
id: fixture_elixir_ocr_error_401
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_ocr_async("{\"document\":{\"type\":\"document_url\",\"url\":\"https://example.com/doc.pdf\"},\"model\":\"mistral/mistral-ocr-latest\"}")

```
