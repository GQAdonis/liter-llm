---
id: fixture_python_smoke_create_response
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
import asyncio
import os
from liter_llm import create_client, CreateResponseRequest
from liter_llm._internal_bindings import CreateResponseRequest

async def main() -> None:
    client = create_client(api_key="test-key")
    request = CreateResponseRequest.from_json("{\"input\":\"Explain quantum computing in one sentence.\",\"model\":\"gpt-4o\"}")
    _ = await client.create_response(request)

asyncio.run(main())

```
