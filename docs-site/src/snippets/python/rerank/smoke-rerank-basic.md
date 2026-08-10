---
id: fixture_python_smoke_rerank_basic
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
import asyncio
import os
from liter_llm import create_client, RerankRequest, RerankDocument
from liter_llm._internal_bindings import RerankRequest

async def main() -> None:
    client = create_client(api_key="test-key")
    request = RerankRequest.from_json("{\"documents\":[\"Machine learning is a subset of AI.\",\"The weather is sunny today.\",\"Deep learning uses neural networks.\"],\"model\":\"rerank-v3.5\",\"query\":\"What is machine learning?\"}")
    _ = await client.rerank(request)

asyncio.run(main())

```
