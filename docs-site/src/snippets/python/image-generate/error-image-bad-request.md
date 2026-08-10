---
id: fixture_python_error_image_bad_request
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
    request = CreateImageRequest.from_json("{\"model\":\"dall-e-3\",\"n\":1,\"prompt\":\"A cat\",\"size\":\"9999x9999\"}")
    _ = await client.image_generate(request)

asyncio.run(main())

```
