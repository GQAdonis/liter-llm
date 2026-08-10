---
id: fixture_c_proxy_rerank
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
    LITERLLMRerankRequest* rerank_request_handle = literllm_rerank_request_from_json("{\"documents\":[\"Deep learning is a subset of machine learning using neural networks.\",\"The stock market closed higher today.\"],\"model\":\"rerank-v3.5\",\"query\":\"What is deep learning?\"}");
    LITERLLMDefaultClient* client = literllm_create_client("test-key", NULL, (uint64_t)-1, (uint32_t)-1, NULL);
    LITERLLMRerankResponse* result = literllm_default_client_rerank(client, rerank_request_handle);
    literllm_rerank_response_free(result);
    literllm_rerank_request_free(rerank_request_handle);
    literllm_default_client_free(client);
    return EXIT_SUCCESS;
}

```
