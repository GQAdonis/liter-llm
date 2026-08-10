---
id: fixture_wasm_edge_stream_function_call
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmChatCompletionRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmChatCompletionRequest = (() => { const _u0 = WasmChatCompletionRequest.default(); _u0.messages = [{ content: "What's the weather?", role: "user" }]; _u0.model = "gpt-4"; _u0.tools = [(() => { const _u1 = WasmChatCompletionTool.default(); _u1.function = (() => { const _u2 = WasmFunctionDefinition.default(); _u2.name = "get_weather"; _u2.parameters = { properties: { city: { type: "string" } }, type: "object" }; return _u2; })(); _u1.type = "function"; return _u1; })()]; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.chatStream(request);
}

void main();

```
