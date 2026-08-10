---
id: fixture_c_smoke_list_models_openai
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
    if (getenv("OPENAI_API_KEY") == NULL) { return; }
    LITERLLMDefaultClient* client = literllm_create_client("test-key", NULL, (uint64_t)-1, (uint32_t)-1, NULL);
    LITERLLMModelsListResponse* result = literllm_default_client_list_models(client);
    literllm_models_list_response_free(result);
    literllm_default_client_free(client);
    return EXIT_SUCCESS;
}

```
