---
id: fixture_wasm_embed_with_dimensions
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmEmbeddingRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmEmbeddingRequest = (() => { const _u0 = WasmEmbeddingRequest.default(); _u0.dimensions = 256; _u0.input = "Hello world"; _u0.model = "text-embedding-3-small"; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.embed(request);
}

void main();

```
