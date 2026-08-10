---
id: fixture_python_error_speech_bad_model
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
    request = CreateSpeechRequest.from_json("{\"input\":\"Hello\",\"model\":\"tts-nonexistent\",\"voice\":\"alloy\"}")
    _ = await client.speech(request)

asyncio.run(main())

```
