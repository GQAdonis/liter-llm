---
id: fixture_c_edge_moderate_empty_input
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
    LITERLLMModerationRequest* moderation_request_handle = literllm_moderation_request_from_json("{\"input\":\"\",\"model\":\"omni-moderation-latest\"}");
    LITERLLMDefaultClient* client = literllm_create_client("test-key", NULL, (uint64_t)-1, (uint32_t)-1, NULL);
    LITERLLMModerationResponse* result = literllm_default_client_moderate(client, moderation_request_handle);
    literllm_moderation_response_free(result);
    literllm_moderation_request_free(moderation_request_handle);
    literllm_default_client_free(client);
    return EXIT_SUCCESS;
}

```
