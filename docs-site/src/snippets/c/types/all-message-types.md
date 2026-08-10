---
id: fixture_c_all_message_types
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
    LITERLLMChatCompletionRequest* chat_completion_request_handle = literllm_chat_completion_request_from_json("{\"messages\":[{\"content\":\"You are a helpful assistant.\",\"role\":\"system\"},{\"content\":\"What is the weather in Paris?\",\"role\":\"user\"},{\"content\":null,\"role\":\"assistant\",\"tool_calls\":[{\"function\":{\"arguments\":\"{\\\"location\\\": \\\"Paris, France\\\"}\",\"name\":\"get_weather\"},\"id\":\"call_xyz789\",\"type\":\"function\"}]},{\"content\":\"{\\\"temperature\\\": 18, \\\"unit\\\": \\\"celsius\\\", \\\"description\\\": \\\"Partly cloudy\\\"}\",\"role\":\"tool\",\"tool_call_id\":\"call_xyz789\"}],\"model\":\"gpt-4\"}");
    LITERLLMDefaultClient* client = literllm_create_client("test-key", NULL, (uint64_t)-1, (uint32_t)-1, NULL);
    LITERLLMChatCompletionResponse* result = literllm_default_client_chat(client, chat_completion_request_handle);
    literllm_chat_completion_response_free(result);
    literllm_chat_completion_request_free(chat_completion_request_handle);
    literllm_default_client_free(client);
    return EXIT_SUCCESS;
}

```
