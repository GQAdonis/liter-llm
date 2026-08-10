---
id: fixture_python_vertex_embed
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
    request = EmbeddingRequest.from_json("{\"input\":\"Hello\",\"model\":\"vertex_ai/text-embedding-005\"}")
    _ = await client.embed(request)

asyncio.run(main())

```
