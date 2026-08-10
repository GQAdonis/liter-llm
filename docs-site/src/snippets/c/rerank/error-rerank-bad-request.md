---
id: fixture_c_error_rerank_bad_request
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
    LITERLLMRerankRequest* rerank_request_handle = literllm_rerank_request_from_json("{\"documents\":[\"doc1\"],\"model\":\"nonexistent-rerank\",\"query\":\"test\"}");
    if (rerank_request_handle == NULL) { return; }
    LITERLLMDefaultClient* client = literllm_create_client("test-key", NULL, (uint64_t)-1, (uint32_t)-1, NULL);
    if (result != NULL) { return EXIT_FAILURE; }
    LITERLLMRerankResponse* result = literllm_default_client_rerank(client, rerank_request_handle);
    literllm_rerank_request_free(rerank_request_handle);
    literllm_default_client_free(client);
    if (result != NULL) { return EXIT_FAILURE; }
    return EXIT_SUCCESS;
}

```
