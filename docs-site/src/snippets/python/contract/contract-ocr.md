---
id: fixture_python_contract_ocr
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
import asyncio
import os
from liter_llm import create_client, OcrRequest, OcrDocument
from liter_llm._internal_bindings import OcrRequest

async def main() -> None:
    client = create_client(api_key="test-key")
    request = OcrRequest.from_json("{\"document\":{\"type\":\"document_url\",\"url\":\"https://example.com/contract-test.pdf\"},\"model\":\"mistral/mistral-ocr-latest\"}")
    _ = await client.ocr(request)

asyncio.run(main())

```
