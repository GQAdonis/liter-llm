---
id: fixture_wasm_error_response_bad_request
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmCreateResponseRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmCreateResponseRequest = (() => { const _u0 = WasmCreateResponseRequest.default(); _u0.input = "Hello"; _u0.model = "nonexistent-model"; return _u0; })();
  const client = createClient("your-api-key");
  try {
    await client.createResponse(request);
  } catch (error) {
    console.error("Call failed as expected:", error);
    return;
  }
  throw new Error("expected call to fail");
}

void main();

```
