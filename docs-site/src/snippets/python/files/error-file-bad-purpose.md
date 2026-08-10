---
id: fixture_python_error_file_bad_purpose
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
import asyncio
import os

async def main() -> None:
    client = create_client(api_key="test-key")
    request = CreateFileRequest.from_json("{\"file\":\"data.jsonl\",\"purpose\":\"invalid-purpose\"}")
    _ = await client.create_file(request)

asyncio.run(main())

```
