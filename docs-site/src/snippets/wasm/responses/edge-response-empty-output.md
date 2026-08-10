---
id: fixture_wasm_edge_response_empty_output
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmCreateResponseRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmCreateResponseRequest = (() => { const _u0 = WasmCreateResponseRequest.default(); _u0.input = ""; _u0.model = "gpt-4o"; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.createResponse(request);
}

void main();

```
