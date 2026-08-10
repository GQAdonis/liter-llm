---
id: fixture_python_edge_rerank_single_doc
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
    request = RerankRequest.from_json("{\"documents\":[\"Artificial intelligence is the simulation of human intelligence.\"],\"model\":\"rerank-v3.5\",\"query\":\"What is AI?\"}")
    _ = await client.rerank(request)

asyncio.run(main())

```
