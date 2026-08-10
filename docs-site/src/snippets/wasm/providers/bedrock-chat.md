---
id: fixture_wasm_bedrock_chat
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmChatCompletionRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmChatCompletionRequest = (() => { const _u0 = WasmChatCompletionRequest.default(); _u0.maxTokens = 16n; _u0.messages = [{ content: "Say hello in one word.", role: "user" }]; _u0.model = "bedrock/anthropic.claude-3-sonnet-20240229-v1:0"; _u0.temperature = 0; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.chat(request);
}

void main();

```
