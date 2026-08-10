---
id: fixture_python_anthropic_error_auth
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
    request = ChatCompletionRequest.from_json("{\"messages\":[{\"content\":\"Hello\",\"role\":\"user\"}],\"model\":\"anthropic/claude-3-5-sonnet-20241022\"}")
    _ = await client.chat(request)

asyncio.run(main())

```
