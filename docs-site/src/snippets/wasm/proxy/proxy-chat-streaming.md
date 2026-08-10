---
id: fixture_wasm_proxy_chat_streaming
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmChatCompletionRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmChatCompletionRequest = (() => { const _u0 = WasmChatCompletionRequest.default(); _u0.messages = [{ content: "Count to 3", role: "user" }]; _u0.model = "openai/gpt-4o"; _u0.stream = true; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.chatStream(request);
}

void main();

```
