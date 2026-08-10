---
id: legacy_elixir_getting_started_basic_chat
language: elixir
target: elixir
level: syntax
requires: []
side_effect: network
---

```elixir
{:ok, client} = LiterLlm.create_client(System.get_env("OPENAI_API_KEY"))

request =
  Jason.encode!(%{
    model: "openai/gpt-4o-mini",
    messages: [%{role: "user", content: "Hello!"}]
  })

{:ok, result} = LiterLlm.defaultclient_chat_async(client, request)
IO.puts(Enum.at(result.choices, 0).message.content)
```
