---
id: fixture_python_smoke_list_models_openai
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
    api_key = os.environ.get("OPENAI_API_KEY")
    if not api_key:  # noqa: SIM102
        pytest.skip("OPENAI_API_KEY not set")
    client = create_client(api_key=api_key)
    _ = await client.list_models()

asyncio.run(main())

```
