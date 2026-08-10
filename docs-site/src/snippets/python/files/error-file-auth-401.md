---
id: fixture_python_error_file_auth_401
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
    _ = await client.list_files()

asyncio.run(main())

```
