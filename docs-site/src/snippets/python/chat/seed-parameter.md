---
id: fixture_python_seed_parameter
language: python
target: python
level: typecheck
requires: []
side_effect: network
---

```python title="Python"
import asyncio
import os
from liter_llm import create_client, ChatCompletionRequest, Message
from liter_llm._internal_bindings import ChatCompletionRequest

async def main() -> None:
    client = create_client(api_key="test-key")
    request = ChatCompletionRequest.from_json("{\"messages\":[{\"content\":\"Pick a random number\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"seed\":42}")
    _ = await client.chat(request)

asyncio.run(main())

```
