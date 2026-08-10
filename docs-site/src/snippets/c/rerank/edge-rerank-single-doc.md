---
id: fixture_c_edge_rerank_single_doc
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
    LITERLLMRerankRequest* rerank_request_handle = literllm_rerank_request_from_json("{\"documents\":[\"Artificial intelligence is the simulation of human intelligence.\"],\"model\":\"rerank-v3.5\",\"query\":\"What is AI?\"}");
    LITERLLMDefaultClient* client = literllm_create_client("test-key", NULL, (uint64_t)-1, (uint32_t)-1, NULL);
    LITERLLMRerankResponse* result = literllm_default_client_rerank(client, rerank_request_handle);
    literllm_rerank_response_free(result);
    literllm_rerank_request_free(rerank_request_handle);
    literllm_default_client_free(client);
    return EXIT_SUCCESS;
}

```
