---
id: fixture_c_smoke_speech_basic
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
    LITERLLMCreateSpeechRequest* create_speech_request_handle = literllm_create_speech_request_from_json("{\"input\":\"Hello, world!\",\"model\":\"tts-1\",\"voice\":\"alloy\"}");
    LITERLLMDefaultClient* client = literllm_create_client("test-key", NULL, (uint64_t)-1, (uint32_t)-1, NULL);
    uint8_t* out_ptr = NULL;
    uintptr_t out_len = 0;
    uintptr_t out_cap = 0;
    int32_t status = literllm_default_client_speech(client, create_speech_request_handle, &out_ptr, &out_len, &out_cap);
    literllm_free_bytes(out_ptr, out_len, out_cap);
    literllm_create_speech_request_free(create_speech_request_handle);
    literllm_default_client_free(client);
    return EXIT_SUCCESS;
}

```
