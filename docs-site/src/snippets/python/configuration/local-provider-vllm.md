---
id: fixture_python_local_provider_vllm
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
    request = ChatCompletionRequest.from_json("{\"messages\":[{\"content\":\"Hello\",\"role\":\"user\"}],\"model\":\"vllm/meta-llama/Llama-3.2-1B\"}")
    _ = await client.chat(request)

asyncio.run(main())

```
