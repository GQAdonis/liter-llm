---
id: legacy_toml_server_server
language: toml
target: toml
level: syntax
requires: []
side_effect: network
---

```toml
[server]
host = "0.0.0.0"
port = 4000
request_timeout_secs = 600
body_limit_bytes = 10_485_760
cors_origins = ["https://app.example.com", "https://admin.example.com"]
```
