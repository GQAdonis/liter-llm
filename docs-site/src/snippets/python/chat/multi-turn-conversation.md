---
id: fixture_python_multi_turn_conversation
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
    request = ChatCompletionRequest.from_json("{\"messages\":[{\"content\":\"You are a helpful assistant.\",\"role\":\"system\"},{\"content\":\"What is 2 + 2?\",\"role\":\"user\"},{\"content\":\"2 + 2 equals 4.\",\"role\":\"assistant\"},{\"content\":\"And what is 4 + 4?\",\"role\":\"user\"}],\"model\":\"gpt-4\"}")
    _ = await client.chat(request)

asyncio.run(main())

```
