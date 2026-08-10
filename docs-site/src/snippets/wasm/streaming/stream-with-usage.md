---
id: fixture_wasm_stream_with_usage
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmChatCompletionRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmChatCompletionRequest = (() => { const _u0 = WasmChatCompletionRequest.default(); _u0.messages = [{ content: "Say hi", role: "user" }]; _u0.model = "gpt-4"; _u0.stream = true; _u0.streamOptions = (() => { const _u1 = WasmStreamOptions.default(); _u1.includeUsage = true; return _u1; })(); return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.chatStream(request);
}

void main();

```
