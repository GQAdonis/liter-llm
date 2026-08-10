---
id: fixture_wasm_search_error_400
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmSearchRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmSearchRequest = (() => { const _u0 = WasmSearchRequest.default(); _u0.model = "brave/web-search"; _u0.query = ""; return _u0; })();
  const client = createClient("your-api-key");
  try {
    await client.search(request);
  } catch (error) {
    console.error("Call failed as expected:", error);
    return;
  }
  throw new Error("expected call to fail");
}

void main();

```
