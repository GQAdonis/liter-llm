---
id: fixture_python_smoke_batch_completed
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
    batch_id = "batch-ghi789"
    _ = await client.retrieve_batch(batch_id)

asyncio.run(main())

```
