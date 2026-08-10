---
id: fixture_wasm_smoke_image_multiple
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmCreateImageRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmCreateImageRequest = (() => { const _u0 = WasmCreateImageRequest.default(); _u0.model = "dall-e-2"; _u0.n = 3; _u0.prompt = "A red bicycle"; _u0.size = "256x256"; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.imageGenerate(request);
}

void main();

```
