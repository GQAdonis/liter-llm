---
id: fixture_python_stream_with_usage
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
    request = ChatCompletionRequest.from_json("{\"messages\":[{\"content\":\"Say hi\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"stream\":true,\"stream_options\":{\"include_usage\":true}}")
    result = client.chat_stream(request)
    chunks = []
    async for chunk in result:
        chunks.append(chunk)

asyncio.run(main())

```
