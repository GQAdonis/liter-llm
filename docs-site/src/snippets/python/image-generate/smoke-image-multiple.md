---
id: fixture_python_smoke_image_multiple
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
import asyncio
import os
from liter_llm import create_client, CreateImageRequest
from liter_llm._internal_bindings import CreateImageRequest

async def main() -> None:
    client = create_client(api_key="test-key")
    request = CreateImageRequest.from_json("{\"model\":\"dall-e-2\",\"n\":3,\"prompt\":\"A red bicycle\",\"size\":\"256x256\"}")
    _ = await client.image_generate(request)

asyncio.run(main())

```
