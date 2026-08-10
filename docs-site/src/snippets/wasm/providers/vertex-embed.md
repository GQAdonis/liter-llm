---
id: fixture_wasm_vertex_embed
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmEmbeddingRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmEmbeddingRequest = (() => { const _u0 = WasmEmbeddingRequest.default(); _u0.input = "Hello"; _u0.model = "vertex_ai/text-embedding-005"; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.embed(request);
}

void main();

```
