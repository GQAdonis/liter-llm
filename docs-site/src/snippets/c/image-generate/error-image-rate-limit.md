---
id: fixture_c_error_image_rate_limit
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
    LITERLLMCreateImageRequest* create_image_request_handle = literllm_create_image_request_from_json("{\"model\":\"dall-e-3\",\"n\":1,\"prompt\":\"A cat\",\"size\":\"1024x1024\"}");
    if (create_image_request_handle == NULL) { return; }
    LITERLLMDefaultClient* client = literllm_create_client("test-key", NULL, (uint64_t)-1, (uint32_t)-1, NULL);
    if (result != NULL) { return EXIT_FAILURE; }
    LITERLLMImagesResponse* result = literllm_default_client_image_generate(client, create_image_request_handle);
    literllm_create_image_request_free(create_image_request_handle);
    literllm_default_client_free(client);
    if (result != NULL) { return EXIT_FAILURE; }
    return EXIT_SUCCESS;
}

```
