---
id: fixture_wasm_all_message_types
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmChatCompletionRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmChatCompletionRequest = (() => { const _u0 = WasmChatCompletionRequest.default(); _u0.messages = [{ content: "You are a helpful assistant.", role: "system" }, { content: "What is the weather in Paris?", role: "user" }, { content: null, role: "assistant", tool_calls: [{ function: { arguments: "{\"location\": \"Paris, France\"}", name: "get_weather" }, id: "call_xyz789", type: "function" }] }, { content: "{\"temperature\": 18, \"unit\": \"celsius\", \"description\": \"Partly cloudy\"}", role: "tool", tool_call_id: "call_xyz789" }]; _u0.model = "gpt-4"; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.chat(request);
}

void main();

```
