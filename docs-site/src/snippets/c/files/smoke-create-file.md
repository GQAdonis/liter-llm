---
id: fixture_c_smoke_create_file
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
    LITERLLMCreateFileRequest* create_file_request_handle = literllm_create_file_request_from_json("{\"file\":\"eyJwcm9tcHQiOiAiaGVsbG8ifQo=\",\"filename\":\"training_data.jsonl\",\"purpose\":\"fine-tune\"}");
    LITERLLMDefaultClient* client = literllm_create_client("test-key", NULL, (uint64_t)-1, (uint32_t)-1, NULL);
    LITERLLMFileObject* result = literllm_default_client_create_file(client, create_file_request_handle);
    literllm_file_object_free(result);
    literllm_create_file_request_free(create_file_request_handle);
    literllm_default_client_free(client);
    return EXIT_SUCCESS;
}

```
