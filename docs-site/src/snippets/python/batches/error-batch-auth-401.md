---
id: fixture_python_error_batch_auth_401
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
import asyncio
import os
from liter_llm import create_client, CreateBatchRequest
from liter_llm._internal_bindings import CreateBatchRequest

async def main() -> None:
    client = create_client(api_key="test-key")
    request = CreateBatchRequest.from_json("{\"completion_window\":\"24h\",\"endpoint\":\"/v1/chat/completions\",\"input_file_id\":\"file-abc123\"}")
    _ = await client.create_batch(request)

asyncio.run(main())

```
