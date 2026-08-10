---
id: fixture_wasm_smoke_streaming_openai
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmChatCompletionRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmChatCompletionRequest = (() => { const _u0 = WasmChatCompletionRequest.default(); _u0.maxTokens = 50n; _u0.messages = [{ content: "Count from 1 to 5.", role: "user" }]; _u0.model = "openai/gpt-4o-mini"; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.chatStream(request);
}

void main();

```
