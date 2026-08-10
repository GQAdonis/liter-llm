---
id: fixture_c_multi_turn_conversation
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
    LITERLLMChatCompletionRequest* chat_completion_request_handle = literllm_chat_completion_request_from_json("{\"messages\":[{\"content\":\"You are a helpful assistant.\",\"role\":\"system\"},{\"content\":\"What is 2 + 2?\",\"role\":\"user\"},{\"content\":\"2 + 2 equals 4.\",\"role\":\"assistant\"},{\"content\":\"And what is 4 + 4?\",\"role\":\"user\"}],\"model\":\"gpt-4\"}");
    LITERLLMDefaultClient* client = literllm_create_client("test-key", NULL, (uint64_t)-1, (uint32_t)-1, NULL);
    LITERLLMChatCompletionResponse* result = literllm_default_client_chat(client, chat_completion_request_handle);
    literllm_chat_completion_response_free(result);
    literllm_chat_completion_request_free(chat_completion_request_handle);
    literllm_default_client_free(client);
    return EXIT_SUCCESS;
}

```
