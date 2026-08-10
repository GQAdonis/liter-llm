---
id: fixture_python_error_response_not_found
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
import asyncio
import os
from liter_llm import create_client

async def main() -> None:
    client = create_client(api_key="test-key")
    response_id = "resp-nonexistent"
    _ = await client.retrieve_response(response_id)

asyncio.run(main())

```
