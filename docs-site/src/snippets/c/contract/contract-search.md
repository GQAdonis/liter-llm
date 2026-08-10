---
id: fixture_c_contract_search
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
    LITERLLMSearchRequest* search_request_handle = literllm_search_request_from_json("{\"model\":\"brave/web-search\",\"query\":\"contract test query\"}");
    LITERLLMDefaultClient* client = literllm_create_client("test-key", NULL, (uint64_t)-1, (uint32_t)-1, NULL);
    LITERLLMSearchResponse* result = literllm_default_client_search(client, search_request_handle);
    literllm_search_response_free(result);
    literllm_search_request_free(search_request_handle);
    literllm_default_client_free(client);
    return EXIT_SUCCESS;
}

```
