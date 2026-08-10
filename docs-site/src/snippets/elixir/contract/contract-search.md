---
id: fixture_elixir_contract_search
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_search_async("{\"model\":\"brave/web-search\",\"query\":\"contract test query\"}")

```
