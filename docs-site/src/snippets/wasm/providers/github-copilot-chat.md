---
id: fixture_wasm_github_copilot_chat
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmChatCompletionRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmChatCompletionRequest = (() => { const _u0 = WasmChatCompletionRequest.default(); _u0.maxTokens = 16n; _u0.messages = [{ content: "Say hello in one word.", role: "user" }]; _u0.model = "github_copilot/gpt-4o"; _u0.temperature = 0; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.chat(request);
}

void main();

```
