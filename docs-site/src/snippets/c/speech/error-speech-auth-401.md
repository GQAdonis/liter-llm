---
id: fixture_c_error_speech_auth_401
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
    LITERLLMCreateSpeechRequest* create_speech_request_handle = literllm_create_speech_request_from_json("{\"input\":\"Hello\",\"model\":\"tts-1\",\"voice\":\"alloy\"}");
    if (create_speech_request_handle == NULL) { return; }
    LITERLLMDefaultClient* client = literllm_create_client("test-key", NULL, (uint64_t)-1, (uint32_t)-1, NULL);
    if (result != NULL) { return EXIT_FAILURE; }
    uint8_t* out_ptr = NULL;
    uintptr_t out_len = 0;
    uintptr_t out_cap = 0;
    int32_t status = literllm_default_client_speech(client, create_speech_request_handle, &out_ptr, &out_len, &out_cap);
    literllm_create_speech_request_free(create_speech_request_handle);
    literllm_default_client_free(client);
    if (result != NULL) { return EXIT_FAILURE; }
    literllm_free_bytes(out_ptr, out_len, out_cap);
    return EXIT_SUCCESS;
}

```
