---
id: fixture_c_smoke_batch_completed
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
    LITERLLMDefaultClient* client = literllm_create_client("test-key", NULL, (uint64_t)-1, (uint32_t)-1, NULL);
    LITERLLMBatchObject* result = literllm_default_client_retrieve_batch(client, "batch-ghi789");
    literllm_batch_object_free(result);
    literllm_default_client_free(client);
    return EXIT_SUCCESS;
}

```
