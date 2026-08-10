---
id: fixture_c_proxy_auth_missing
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
    LITERLLMChatCompletionRequest* chat_completion_request_handle = literllm_chat_completion_request_from_json("{\"messages\":[{\"content\":\"Hello\",\"role\":\"user\"}],\"model\":\"openai/gpt-4o\"}");
    if (chat_completion_request_handle == NULL) { return; }
    LITERLLMDefaultClient* client = literllm_create_client("test-key", NULL, (uint64_t)-1, (uint32_t)-1, NULL);
    if (result != NULL) { return EXIT_FAILURE; }
    LITERLLMChatCompletionResponse* result = literllm_default_client_chat(client, chat_completion_request_handle);
    literllm_chat_completion_request_free(chat_completion_request_handle);
    literllm_default_client_free(client);
    if (result != NULL) { return EXIT_FAILURE; }
    return EXIT_SUCCESS;
}

```
