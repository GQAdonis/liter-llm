---
id: fixture_wasm_edge_moderate_empty_input
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmModerationRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmModerationRequest = (() => { const _u0 = WasmModerationRequest.default(); _u0.input = ""; _u0.model = "omni-moderation-latest"; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.moderate(request);
}

void main();

```
