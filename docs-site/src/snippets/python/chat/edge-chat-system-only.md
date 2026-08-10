---
id: fixture_python_edge_chat_system_only
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
    request = ChatCompletionRequest.from_json("{\"messages\":[{\"content\":\"You are a helpful and concise assistant\",\"role\":\"system\"},{\"content\":\"Hi\",\"role\":\"user\"}],\"model\":\"gpt-4\"}")
    _ = await client.chat(request)

asyncio.run(main())

```
