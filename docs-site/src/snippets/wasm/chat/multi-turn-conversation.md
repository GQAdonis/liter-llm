---
id: fixture_wasm_multi_turn_conversation
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmChatCompletionRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmChatCompletionRequest = (() => { const _u0 = WasmChatCompletionRequest.default(); _u0.messages = [{ content: "You are a helpful assistant.", role: "system" }, { content: "What is 2 + 2?", role: "user" }, { content: "2 + 2 equals 4.", role: "assistant" }, { content: "And what is 4 + 4?", role: "user" }]; _u0.model = "gpt-4"; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.chat(request);
}

void main();

```
