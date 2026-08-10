---
id: fixture_wasm_error_batch_invalid_file
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmCreateBatchRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmCreateBatchRequest = (() => { const _u0 = WasmCreateBatchRequest.default(); _u0.completionWindow = "24h"; _u0.endpoint = "/v1/chat/completions"; _u0.inputFileId = "file-wrong-purpose"; return _u0; })();
  const client = createClient("your-api-key");
  try {
    await client.createBatch(request);
  } catch (error) {
    console.error("Call failed as expected:", error);
    return;
  }
  throw new Error("expected call to fail");
}

void main();

```
