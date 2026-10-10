import { registerBundledContent } from "../ai-rulez-content.js";

export default {
  id: "liter-llm",
  async setup(ctx) {
    await registerBundledContent(ctx);
  },
};
