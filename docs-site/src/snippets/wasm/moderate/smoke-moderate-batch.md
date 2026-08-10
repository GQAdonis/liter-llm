---
id: fixture_wasm_smoke_moderate_batch
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmModerationRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmModerationRequest = (() => { const _u0 = WasmModerationRequest.default(); _u0.input = ["Hello world", "Nice weather today"]; _u0.model = "omni-moderation-latest"; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.moderate(request);
}

void main();

```
