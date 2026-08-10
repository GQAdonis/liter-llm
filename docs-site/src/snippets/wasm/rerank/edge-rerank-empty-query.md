---
id: fixture_wasm_edge_rerank_empty_query
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmRerankRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmRerankRequest = (() => { const _u0 = WasmRerankRequest.default(); _u0.documents = ["Some document", "Another document"]; _u0.model = "rerank-v3.5"; _u0.query = ""; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.rerank(request);
}

void main();

```
