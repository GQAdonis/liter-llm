---
id: fixture_wasm_response_format_json_schema
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmChatCompletionRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmChatCompletionRequest = (() => { const _u0 = WasmChatCompletionRequest.default(); _u0.messages = [{ content: "What is the temperature in Paris today?", role: "user" }]; _u0.model = "gpt-4"; _u0.responseFormat = { jsonSchema: { name: "weather", schema: { properties: { temp: { type: "number" } }, required: ["temp"], type: "object" } }, type: "json_schema" }; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.chat(request);
}

void main();

```
