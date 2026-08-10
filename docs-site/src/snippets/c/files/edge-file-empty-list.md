---
id: fixture_c_edge_file_empty_list
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
    LITERLLMFileListResponse* result = literllm_default_client_list_files(client, NULL);
    literllm_file_list_response_free(result);
    literllm_default_client_free(client);
    return EXIT_SUCCESS;
}

```
