---
id: fixture_wasm_ocr_url_document
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmOcrRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmOcrRequest = (() => { const _u0 = WasmOcrRequest.default(); _u0.document = { type: "document_url", url: "https://example.com/doc.pdf" }; _u0.model = "mistral/mistral-ocr-latest"; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.ocr(request);
}

void main();

```
