---
id: fixture_wasm_seed_parameter
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: network
---

```typescript title="WebAssembly"
import { WasmChatCompletionRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmChatCompletionRequest = (() => { const _u0 = WasmChatCompletionRequest.default(); _u0.messages = [{ content: "Pick a random number", role: "user" }]; _u0.model = "gpt-4"; _u0.seed = 42n; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.chat(request);
}

void main();

```
