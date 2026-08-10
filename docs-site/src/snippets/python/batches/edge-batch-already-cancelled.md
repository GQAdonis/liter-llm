---
id: fixture_python_edge_batch_already_cancelled
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
    batch_id = "batch-cancelled001"
    _ = await client.cancel_batch(batch_id)

asyncio.run(main())

```
