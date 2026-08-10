---
id: fixture_c_contract_ocr
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
    LITERLLMOcrRequest* ocr_request_handle = literllm_ocr_request_from_json("{\"document\":{\"type\":\"document_url\",\"url\":\"https://example.com/contract-test.pdf\"},\"model\":\"mistral/mistral-ocr-latest\"}");
    LITERLLMDefaultClient* client = literllm_create_client("test-key", NULL, (uint64_t)-1, (uint32_t)-1, NULL);
    LITERLLMOcrResponse* result = literllm_default_client_ocr(client, ocr_request_handle);
    literllm_ocr_response_free(result);
    literllm_ocr_request_free(ocr_request_handle);
    literllm_default_client_free(client);
    return EXIT_SUCCESS;
}

```
