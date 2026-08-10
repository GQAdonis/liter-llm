---
id: fixture_wasm_anthropic_chat
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmChatCompletionRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmChatCompletionRequest = (() => { const _u0 = WasmChatCompletionRequest.default(); _u0.maxTokens = 16n; _u0.messages = [{ content: "You are a helpful assistant.", role: "system" }, { content: "Say hello in one word.", role: "user" }]; _u0.model = "anthropic/claude-3-5-sonnet-20241022"; _u0.temperature = 0; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.chat(request);
}

void main();

```
