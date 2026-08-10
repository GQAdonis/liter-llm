---
id: fixture_python_smoke_rerank_return_docs
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
    request = RerankRequest.from_json("{\"documents\":[\"Rust is a systems programming language.\",\"Iron rusts when exposed to water.\"],\"model\":\"rerank-v3.5\",\"query\":\"What is Rust?\",\"return_documents\":true}")
    _ = await client.rerank(request)

asyncio.run(main())

```
