---
id: fixture_python_smoke_response_with_tools
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
import asyncio
import os
from liter_llm import create_client, CreateResponseRequest
from liter_llm._internal_bindings import CreateResponseRequest

async def main() -> None:
    client = create_client(api_key="test-key")
    request = CreateResponseRequest.from_json("{\"input\":\"What is the weather in San Francisco?\",\"model\":\"gpt-4o\",\"tools\":[{\"description\":\"Get current weather for a location\",\"name\":\"get_weather\",\"parameters\":{\"properties\":{\"location\":{\"type\":\"string\"}},\"required\":[\"location\"],\"type\":\"object\"},\"type\":\"function\"}]}")
    _ = await client.create_response(request)

asyncio.run(main())

```
