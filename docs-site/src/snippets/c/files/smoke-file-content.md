---
id: fixture_c_smoke_file_content
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
    uint8_t* out_ptr = NULL;
    uintptr_t out_len = 0;
    uintptr_t out_cap = 0;
    int32_t status = literllm_default_client_file_content(client, "file-abc123", &out_ptr, &out_len, &out_cap);
    literllm_free_bytes(out_ptr, out_len, out_cap);
    literllm_default_client_free(client);
    return EXIT_SUCCESS;
}

```
