---
id: fixture_c_error_moderate_auth_401
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
    LITERLLMModerationRequest* moderation_request_handle = literllm_moderation_request_from_json("{\"input\":\"Hello\",\"model\":\"omni-moderation-latest\"}");
    if (moderation_request_handle == NULL) { return; }
    LITERLLMDefaultClient* client = literllm_create_client("test-key", NULL, (uint64_t)-1, (uint32_t)-1, NULL);
    if (result != NULL) { return EXIT_FAILURE; }
    LITERLLMModerationResponse* result = literllm_default_client_moderate(client, moderation_request_handle);
    literllm_moderation_request_free(moderation_request_handle);
    literllm_default_client_free(client);
    if (result != NULL) { return EXIT_FAILURE; }
    return EXIT_SUCCESS;
}

```
