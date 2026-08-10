---
id: fixture_wasm_ocr_error_400
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmOcrRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmOcrRequest = (() => { const _u0 = WasmOcrRequest.default(); _u0.document = { type: "document_url", url: "invalid://url" }; _u0.model = "mistral/mistral-ocr-latest"; return _u0; })();
  const client = createClient("your-api-key");
  try {
    await client.ocr(request);
  } catch (error) {
    console.error("Call failed as expected:", error);
    return;
  }
  throw new Error("expected call to fail");
}

void main();

```
