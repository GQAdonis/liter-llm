---
id: fixture_c_smoke_rerank_with_top_n
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
    LITERLLMRerankRequest* rerank_request_handle = literllm_rerank_request_from_json("{\"documents\":[\"Python is a programming language.\",\"Cats are cute animals.\",\"Python was created by Guido van Rossum.\",\"The sun is a star.\"],\"model\":\"rerank-v3.5\",\"query\":\"What is Python?\",\"top_n\":2}");
    LITERLLMDefaultClient* client = literllm_create_client("test-key", NULL, (uint64_t)-1, (uint32_t)-1, NULL);
    LITERLLMRerankResponse* result = literllm_default_client_rerank(client, rerank_request_handle);
    literllm_rerank_response_free(result);
    literllm_rerank_request_free(rerank_request_handle);
    literllm_default_client_free(client);
    return EXIT_SUCCESS;
}

```
