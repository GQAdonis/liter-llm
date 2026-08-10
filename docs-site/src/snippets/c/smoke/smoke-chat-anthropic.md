---
id: fixture_c_smoke_chat_anthropic
language: c
target: c
level: typecheck
requires: []
side_effect: safe
---

```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "liter_llm.h"

int main(void) {
    if (getenv("ANTHROPIC_API_KEY") == NULL) { return; }
    LITERLLMChatCompletionRequest* chat_completion_request_handle = literllm_chat_completion_request_from_json("{\"max_tokens\":10,\"messages\":[{\"content\":\"Say hello in exactly one word.\",\"role\":\"user\"}],\"model\":\"anthropic/claude-sonnet-4-20250514\"}");
    LITERLLMDefaultClient* client = literllm_create_client("test-key", NULL, (uint64_t)-1, (uint32_t)-1, NULL);
    LITERLLMChatCompletionResponse* result = literllm_default_client_chat(client, chat_completion_request_handle);
    literllm_chat_completion_response_free(result);
    literllm_chat_completion_request_free(chat_completion_request_handle);
    literllm_default_client_free(client);
    return EXIT_SUCCESS;
}

```
