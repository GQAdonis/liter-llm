---
id: fixture_wasm_error_image_auth_401
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmCreateImageRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmCreateImageRequest = (() => { const _u0 = WasmCreateImageRequest.default(); _u0.model = "dall-e-3"; _u0.n = 1; _u0.prompt = "A cat"; _u0.size = "1024x1024"; return _u0; })();
  const client = createClient("your-api-key");
  try {
    await client.imageGenerate(request);
  } catch (error) {
    console.error("Call failed as expected:", error);
    return;
  }
  throw new Error("expected call to fail");
}

void main();

```
