---
id: fixture_wasm_smoke_image_with_size
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmCreateImageRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmCreateImageRequest = (() => { const _u0 = WasmCreateImageRequest.default(); _u0.model = "dall-e-3"; _u0.n = 1; _u0.prompt = "A sunset over mountains"; _u0.size = "1792x1024"; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.imageGenerate(request);
}

void main();

```
