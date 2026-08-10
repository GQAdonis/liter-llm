---
id: legacy_toml_server_general
language: toml
target: toml
level: syntax
requires: []
side_effect: network
---

```toml
[general]
master_key = "${LITER_LLM_MASTER_KEY}"
default_timeout_secs = 120
max_retries = 3
enable_cost_tracking = true
enable_tracing = true
```
