---
id: fixture_python_search_with_max_results
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
import asyncio
import os
from liter_llm import create_client, SearchRequest
from liter_llm._internal_bindings import SearchRequest

async def main() -> None:
    client = create_client(api_key="test-key")
    request = SearchRequest.from_json("{\"max_results\":2,\"model\":\"brave/web-search\",\"query\":\"Rust programming\"}")
    _ = await client.search(request)

asyncio.run(main())

```
