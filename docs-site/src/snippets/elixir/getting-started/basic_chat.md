---
id: readme_elixir_basic_chat
language: elixir
target: elixir
level: syntax
requires: []
side_effect: network
---

Send a message to any provider using the `provider/model` prefix.

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
