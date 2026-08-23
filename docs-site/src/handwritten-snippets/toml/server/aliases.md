---
id: legacy_toml_server_aliases
language: toml
target: toml
level: syntax
requires: []
side_effect: network
---

```toml
# Any model matching "anthropic/*" uses the shared Anthropic key.
[[aliases]]
pattern = "anthropic/*"
api_key = "${ANTHROPIC_API_KEY}"

# Route all OpenAI models through an Azure deployment.
[[aliases]]
pattern = "openai/*"
api_key = "${AZURE_OPENAI_KEY}"
base_url = "https://my-azure.openai.azure.com"
```
