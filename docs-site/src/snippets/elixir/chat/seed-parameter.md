---
id: fixture_elixir_seed_parameter
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: network
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"messages\":[{\"content\":\"Pick a random number\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"seed\":42}")

```
