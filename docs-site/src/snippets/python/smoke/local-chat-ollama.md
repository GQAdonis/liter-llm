---
id: fixture_python_local_chat_ollama
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
import asyncio
import os

async def main() -> None:
    client = create_client(api_key="test-key")
    request = ChatCompletionRequest.from_json("{\"max_tokens\":10,\"messages\":[{\"content\":\"Say hello in one word.\",\"role\":\"user\"}],\"model\":\"ollama/qwen2:0.5b\"}")
    _ = await client.chat(request)

asyncio.run(main())

```
