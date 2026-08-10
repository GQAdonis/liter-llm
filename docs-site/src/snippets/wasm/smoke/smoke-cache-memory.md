---
id: fixture_wasm_smoke_cache_memory
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmChatCompletionRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmChatCompletionRequest = (() => { const _u0 = WasmChatCompletionRequest.default(); _u0.maxTokens = 5n; _u0.messages = [{ content: "What is 2+2? Answer with just the number.", role: "user" }]; _u0.model = "openai/gpt-4o-mini"; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.chat(request);
}

void main();

```
