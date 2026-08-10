---
id: fixture_python_tool_choice_required
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
import asyncio
import os
from liter_llm import create_client, ChatCompletionRequest, Message, ToolChoice
from liter_llm._internal_bindings import ChatCompletionRequest

async def main() -> None:
    client = create_client(api_key="test-key")
    request = ChatCompletionRequest.from_json("{\"messages\":[{\"content\":\"What is the weather today?\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"tool_choice\":\"required\",\"tools\":[{\"function\":{\"description\":\"Get the current weather for a given location\",\"name\":\"get_weather\",\"parameters\":{\"properties\":{\"location\":{\"description\":\"The city name\",\"type\":\"string\"}},\"required\":[\"location\"],\"type\":\"object\"}},\"type\":\"function\"}]}")
    _ = await client.chat(request)

asyncio.run(main())

```
