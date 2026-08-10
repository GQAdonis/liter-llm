---
id: fixture_c_edge_batch_already_cancelled
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
    if (result != NULL) { return EXIT_FAILURE; }
    LITERLLMBatchObject* result = literllm_default_client_cancel_batch(client, "batch-cancelled001");
    literllm_default_client_free(client);
    if (result != NULL) { return EXIT_FAILURE; }
    return EXIT_SUCCESS;
}

```
