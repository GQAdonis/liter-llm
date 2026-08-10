---
id: fixture_python_smoke_file_content
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
    file_id = "file-abc123"
    _ = await client.file_content(file_id)

asyncio.run(main())

```
