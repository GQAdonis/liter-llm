---
id: fixture_wasm_smoke_create_response
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmCreateResponseRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmCreateResponseRequest = (() => { const _u0 = WasmCreateResponseRequest.default(); _u0.input = "Explain quantum computing in one sentence."; _u0.model = "gpt-4o"; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.createResponse(request);
}

void main();

```
