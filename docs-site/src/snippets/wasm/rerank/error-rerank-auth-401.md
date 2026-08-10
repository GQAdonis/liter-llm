---
id: fixture_wasm_error_rerank_auth_401
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmRerankRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmRerankRequest = (() => { const _u0 = WasmRerankRequest.default(); _u0.documents = ["doc1"]; _u0.model = "rerank-v3.5"; _u0.query = "test"; return _u0; })();
  const client = createClient("your-api-key");
  try {
    await client.rerank(request);
  } catch (error) {
    console.error("Call failed as expected:", error);
    return;
  }
  throw new Error("expected call to fail");
}

void main();

```
