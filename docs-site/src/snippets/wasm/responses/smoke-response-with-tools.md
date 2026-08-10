---
id: fixture_wasm_smoke_response_with_tools
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmCreateResponseRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmCreateResponseRequest = (() => { const _u0 = WasmCreateResponseRequest.default(); _u0.input = "What is the weather in San Francisco?"; _u0.model = "gpt-4o"; _u0.tools = [(() => { const _u1 = WasmResponseTool.default(); _u1.description = "Get current weather for a location"; _u1.name = "get_weather"; _u1.parameters = { properties: { location: { type: "string" } }, required: ["location"], type: "object" }; _u1.type = "function"; return _u1; })()]; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.createResponse(request);
}

void main();

```
