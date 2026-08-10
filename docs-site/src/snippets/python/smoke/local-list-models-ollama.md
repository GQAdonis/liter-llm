---
id: fixture_python_local_list_models_ollama
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
import asyncio
import os

async def main() -> None:
    client = create_client(api_key="test-key")
    _ = await client.list_models()

asyncio.run(main())

```
