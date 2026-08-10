---
id: fixture_wasm_error_moderate_auth_401
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmModerationRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmModerationRequest = (() => { const _u0 = WasmModerationRequest.default(); _u0.input = "Hello"; _u0.model = "omni-moderation-latest"; return _u0; })();
  const client = createClient("your-api-key");
  try {
    await client.moderate(request);
  } catch (error) {
    console.error("Call failed as expected:", error);
    return;
  }
  throw new Error("expected call to fail");
}

void main();

```
