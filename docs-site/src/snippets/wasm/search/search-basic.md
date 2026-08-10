---
id: fixture_wasm_search_basic
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmSearchRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmSearchRequest = (() => { const _u0 = WasmSearchRequest.default(); _u0.model = "brave/web-search"; _u0.query = "What is Rust programming language?"; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.search(request);
}

void main();

```
