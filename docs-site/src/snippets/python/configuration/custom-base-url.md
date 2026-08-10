---
id: fixture_python_custom_base_url
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
import asyncio
import os
from liter_llm import create_client, ChatCompletionRequest, Message
from liter_llm._internal_bindings import ChatCompletionRequest

async def main() -> None:
    client = create_client(api_key="test-key")
    request = ChatCompletionRequest.from_json("{\"messages\":[{\"content\":\"Hello\",\"role\":\"user\"}],\"model\":\"local-model\"}")
    _ = await client.chat(request)

asyncio.run(main())

```
