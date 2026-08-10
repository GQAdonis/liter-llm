---
id: fixture_wasm_anthropic_stream
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmChatCompletionRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmChatCompletionRequest = (() => { const _u0 = WasmChatCompletionRequest.default(); _u0.maxTokens = 32n; _u0.messages = [{ content: "Count to three, one word per response.", role: "user" }]; _u0.model = "anthropic/claude-3-5-sonnet-20241022"; _u0.stream = true; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.chatStream(request);
}

void main();

```
