---
id: fixture_c_embed_error_401
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
    LITERLLMEmbeddingRequest* embedding_request_handle = literllm_embedding_request_from_json("{\"input\":\"Hello world\",\"model\":\"text-embedding-3-small\"}");
    if (embedding_request_handle == NULL) { return; }
    LITERLLMDefaultClient* client = literllm_create_client("test-key", NULL, (uint64_t)-1, (uint32_t)-1, NULL);
    if (result != NULL) { return EXIT_FAILURE; }
    LITERLLMEmbeddingResponse* result = literllm_default_client_embed(client, embedding_request_handle);
    literllm_embedding_request_free(embedding_request_handle);
    literllm_default_client_free(client);
    if (result != NULL) { return EXIT_FAILURE; }
    return EXIT_SUCCESS;
}

```
