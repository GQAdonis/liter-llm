---
id: fixture_python_azure_stream
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
    request = ChatCompletionRequest.from_json("{\"messages\":[{\"content\":\"Count to 3\",\"role\":\"user\"}],\"model\":\"azure/gpt-4\",\"stream\":true,\"temperature\":0}")
    result = client.chat_stream(request)
    chunks = []
    async for chunk in result:
        chunks.append(chunk)

asyncio.run(main())

```
