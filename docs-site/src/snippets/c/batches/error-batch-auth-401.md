---
id: fixture_c_error_batch_auth_401
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
    LITERLLMCreateBatchRequest* create_batch_request_handle = literllm_create_batch_request_from_json("{\"completion_window\":\"24h\",\"endpoint\":\"/v1/chat/completions\",\"input_file_id\":\"file-abc123\"}");
    if (create_batch_request_handle == NULL) { return; }
    LITERLLMDefaultClient* client = literllm_create_client("test-key", NULL, (uint64_t)-1, (uint32_t)-1, NULL);
    if (result != NULL) { return EXIT_FAILURE; }
    LITERLLMBatchObject* result = literllm_default_client_create_batch(client, create_batch_request_handle);
    literllm_create_batch_request_free(create_batch_request_handle);
    literllm_default_client_free(client);
    if (result != NULL) { return EXIT_FAILURE; }
    return EXIT_SUCCESS;
}

```
