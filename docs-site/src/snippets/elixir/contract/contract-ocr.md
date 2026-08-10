---
id: fixture_elixir_contract_ocr
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_ocr_async("{\"document\":{\"type\":\"document_url\",\"url\":\"https://example.com/contract-test.pdf\"},\"model\":\"mistral/mistral-ocr-latest\"}")

```
