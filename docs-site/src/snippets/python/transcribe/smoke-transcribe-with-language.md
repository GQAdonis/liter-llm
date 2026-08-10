---
id: fixture_python_smoke_transcribe_with_language
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
import asyncio
import os
from liter_llm import create_client, CreateTranscriptionRequest
from liter_llm._internal_bindings import CreateTranscriptionRequest

async def main() -> None:
    client = create_client(api_key="test-key")
    request = CreateTranscriptionRequest.from_json("{\"file\":\"audio_de.mp3\",\"language\":\"de\",\"model\":\"whisper-1\"}")
    _ = await client.transcribe(request)

asyncio.run(main())

```
