---
id: fixture_python_multimodal_content
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
    request = ChatCompletionRequest.from_json("{\"max_tokens\":100,\"messages\":[{\"content\":[{\"text\":\"What is in this image?\",\"type\":\"text\"},{\"image_url\":{\"detail\":\"low\",\"url\":\"https://upload.wikimedia.org/wikipedia/commons/thumb/4/47/PNG_transparency_demonstration_1.png/280px-PNG_transparency_demonstration_1.png\"},\"type\":\"image_url\"}],\"role\":\"user\"}],\"model\":\"gpt-4o\"}")
    _ = await client.chat(request)

asyncio.run(main())

```
