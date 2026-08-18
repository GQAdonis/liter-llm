---
id: legacy_curl_server_batches_create
language: bash
target: bash
level: syntax
requires: []
side_effect: network
---

```bash
curl http://localhost:4000/v1/batches \
  -H "Authorization: Bearer $LITER_LLM_MASTER_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "input_file_id": "file_abc123",
    "endpoint": "/v1/chat/completions",
    "completion_window": "24h"
  }'
```
