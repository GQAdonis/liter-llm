---
id: fixture_elixir_ocr_error_400
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_ocr_async("{\"document\":{\"type\":\"document_url\",\"url\":\"invalid://url\"},\"model\":\"mistral/mistral-ocr-latest\"}")

```
