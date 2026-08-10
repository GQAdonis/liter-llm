---
id: fixture_python_proxy_moderation
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
import asyncio
import os
from liter_llm import create_client, ModerationRequest, ModerationInput
from liter_llm._internal_bindings import ModerationRequest

async def main() -> None:
    client = create_client(api_key="test-key")
    request = ModerationRequest.from_json("{\"input\":\"The weather is nice today.\",\"model\":\"omni-moderation-latest\"}")
    _ = await client.moderate(request)

asyncio.run(main())

```
