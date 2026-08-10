---
id: fixture_python_embed_error_401
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
import asyncio
import os
from liter_llm import create_client, EmbeddingRequest, EmbeddingInput
from liter_llm._internal_bindings import EmbeddingRequest

async def main() -> None:
    client = create_client(api_key="test-key")
    request = EmbeddingRequest.from_json("{\"input\":\"Hello world\",\"model\":\"text-embedding-3-small\"}")
    _ = await client.embed(request)

asyncio.run(main())

```
