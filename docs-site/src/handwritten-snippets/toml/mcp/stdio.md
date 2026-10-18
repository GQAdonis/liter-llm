---
id: legacy_toml_mcp_stdio
language: toml
target: toml
level: syntax
requires: []
side_effect: network
---

```toml title="liter-llm-proxy.toml"
[mcp]
stdio_trust_local = true
```

Local stdio clients are trusted by default; this table makes the default explicit. To enforce a virtual-key policy, set `stdio_trust_local = false` and `stdio_key_id` to an existing `[[keys]].key`.

```json title="claude_desktop_config.json (stdio)"
{
  "mcpServers": {
    "liter-llm": {
      "command": "liter-llm",
      "args": ["mcp", "--config", "/absolute/path/to/liter-llm-proxy.toml"],
      "env": {
        "OPENAI_API_KEY": "sk-...",
        "ANTHROPIC_API_KEY": "sk-ant-..."
      }
    }
  }
}
```
