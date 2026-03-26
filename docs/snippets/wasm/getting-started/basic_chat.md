```typescript
import init, { LlmClient } from "liter-lm-wasm";

await init();

const client = new LlmClient();
const response = await client.chat({
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "Hello!" }],
});

console.log(response.content);
```
