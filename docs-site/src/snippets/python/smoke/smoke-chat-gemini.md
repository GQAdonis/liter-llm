---
id: fixture_python_smoke_chat_gemini
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
    api_key = os.environ.get("GEMINI_API_KEY")
    if not api_key:  # noqa: SIM102
        pytest.skip("GEMINI_API_KEY not set")
    client = create_client(api_key=api_key)
    request = ChatCompletionRequest.from_json("{\"max_tokens\":10,\"messages\":[{\"content\":\"Say hello in exactly one word.\",\"role\":\"user\"}],\"model\":\"gemini/gemini-2.5-flash-lite\"}")
    _ = await client.chat(request)

asyncio.run(main())

```
