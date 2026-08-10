---
id: fixture_python_smoke_speech_basic
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
import asyncio
import os
from liter_llm import create_client, CreateSpeechRequest
from liter_llm._internal_bindings import CreateSpeechRequest

async def main() -> None:
    client = create_client(api_key="test-key")
    request = CreateSpeechRequest.from_json("{\"input\":\"Hello, world!\",\"model\":\"tts-1\",\"voice\":\"alloy\"}")
    _ = await client.speech(request)

asyncio.run(main())

```
