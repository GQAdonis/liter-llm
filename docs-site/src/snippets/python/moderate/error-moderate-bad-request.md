---
id: fixture_python_error_moderate_bad_request
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
    request = ModerationRequest.from_json("{\"input\":\"Hello\",\"model\":\"nonexistent-moderation\"}")
    _ = await client.moderate(request)

asyncio.run(main())

```
