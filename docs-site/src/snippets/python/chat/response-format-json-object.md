---
id: fixture_python_response_format_json_object
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
import asyncio
import os
from liter_llm import create_client, ChatCompletionRequest, Message, ResponseFormat
from liter_llm._internal_bindings import ChatCompletionRequest

async def main() -> None:
    client = create_client(api_key="test-key")
    request = ChatCompletionRequest.from_json("{\"messages\":[{\"content\":\"Respond with JSON only.\",\"role\":\"system\"},{\"content\":\"Give me a user object with name and age fields.\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"response_format\":{\"type\":\"json_object\"}}")
    _ = await client.chat(request)

asyncio.run(main())

```
