---
id: fixture_python_edge_file_large_upload
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
import asyncio
import os
from liter_llm import create_client, CreateFileRequest, FilePurpose
from liter_llm._internal_bindings import CreateFileRequest

async def main() -> None:
    client = create_client(api_key="test-key")
    request = CreateFileRequest.from_json("{\"file\":\"eyJwcm9tcHQiOiAibGFyZ2UgdHJhaW5pbmcgZGF0YSJ9Cg==\",\"filename\":\"large_training_data.jsonl\",\"purpose\":\"fine-tune\"}")
    _ = await client.create_file(request)

asyncio.run(main())

```
