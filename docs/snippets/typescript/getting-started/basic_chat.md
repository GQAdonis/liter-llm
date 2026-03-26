```typescript
import { LlmClient } from "liter-lm";

const client = new LlmClient();
const response = await client.chat({
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "Hello!" }],
});
console.log(response.content);
```
