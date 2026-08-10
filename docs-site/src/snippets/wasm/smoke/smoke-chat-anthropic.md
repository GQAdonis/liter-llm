---
id: fixture_wasm_smoke_chat_anthropic
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmChatCompletionRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmChatCompletionRequest = (() => { const _u0 = WasmChatCompletionRequest.default(); _u0.maxTokens = 10n; _u0.messages = [{ content: "Say hello in exactly one word.", role: "user" }]; _u0.model = "anthropic/claude-sonnet-4-20250514"; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.chat(request);
}

void main();

```
