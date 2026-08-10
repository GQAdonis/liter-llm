---
id: fixture_wasm_smoke_create_batch
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmCreateBatchRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmCreateBatchRequest = (() => { const _u0 = WasmCreateBatchRequest.default(); _u0.completionWindow = "24h"; _u0.endpoint = "/v1/chat/completions"; _u0.inputFileId = "file-abc123"; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.createBatch(request);
}

void main();

```
