---
id: fixture_c_edge_response_empty_output
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
    LITERLLMCreateResponseRequest* create_response_request_handle = literllm_create_response_request_from_json("{\"input\":\"\",\"model\":\"gpt-4o\"}");
    LITERLLMDefaultClient* client = literllm_create_client("test-key", NULL, (uint64_t)-1, (uint32_t)-1, NULL);
    LITERLLMResponseObject* result = literllm_default_client_create_response(client, create_response_request_handle);
    literllm_response_object_free(result);
    literllm_create_response_request_free(create_response_request_handle);
    literllm_default_client_free(client);
    return EXIT_SUCCESS;
}

```
