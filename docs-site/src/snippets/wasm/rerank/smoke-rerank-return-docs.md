---
id: fixture_wasm_smoke_rerank_return_docs
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmRerankRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmRerankRequest = (() => { const _u0 = WasmRerankRequest.default(); _u0.documents = ["Rust is a systems programming language.", "Iron rusts when exposed to water."]; _u0.model = "rerank-v3.5"; _u0.query = "What is Rust?"; _u0.returnDocuments = true; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.rerank(request);
}

void main();

```
