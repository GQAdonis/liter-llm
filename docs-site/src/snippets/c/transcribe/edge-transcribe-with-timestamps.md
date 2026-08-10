---
id: fixture_c_edge_transcribe_with_timestamps
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
    LITERLLMCreateTranscriptionRequest* create_transcription_request_handle = literllm_create_transcription_request_from_json("{\"file\":\"audio.mp3\",\"model\":\"whisper-1\",\"response_format\":\"verbose_json\"}");
    LITERLLMDefaultClient* client = literllm_create_client("test-key", NULL, (uint64_t)-1, (uint32_t)-1, NULL);
    LITERLLMTranscriptionResponse* result = literllm_default_client_transcribe(client, create_transcription_request_handle);
    literllm_transcription_response_free(result);
    literllm_create_transcription_request_free(create_transcription_request_handle);
    literllm_default_client_free(client);
    return EXIT_SUCCESS;
}

```
