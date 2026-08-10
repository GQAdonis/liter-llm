---
id: fixture_python_smoke_embed_openai
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
import asyncio
import os
from liter_llm import create_client, EmbeddingRequest, EmbeddingInput
from liter_llm._internal_bindings import EmbeddingRequest

async def main() -> None:
    api_key = os.environ.get("OPENAI_API_KEY")
    if not api_key:  # noqa: SIM102
        pytest.skip("OPENAI_API_KEY not set")
    client = create_client(api_key=api_key)
    request = EmbeddingRequest.from_json("{\"input\":[\"Hello world\"],\"model\":\"openai/text-embedding-3-small\"}")
    _ = await client.embed(request)

asyncio.run(main())

```
