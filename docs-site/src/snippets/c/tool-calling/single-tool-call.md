---
id: fixture_c_single_tool_call
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
    LITERLLMChatCompletionRequest* chat_completion_request_handle = literllm_chat_completion_request_from_json("{\"messages\":[{\"content\":\"What is the weather in San Francisco?\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"tool_choice\":\"auto\",\"tools\":[{\"function\":{\"description\":\"Get the current weather for a given location\",\"name\":\"get_weather\",\"parameters\":{\"properties\":{\"location\":{\"description\":\"The city and state, e.g. San Francisco, CA\",\"type\":\"string\"},\"unit\":{\"description\":\"The temperature unit to use\",\"enum\":[\"celsius\",\"fahrenheit\"],\"type\":\"string\"}},\"required\":[\"location\"],\"type\":\"object\"}},\"type\":\"function\"}]}");
    LITERLLMDefaultClient* client = literllm_create_client("test-key", NULL, (uint64_t)-1, (uint32_t)-1, NULL);
    LITERLLMChatCompletionResponse* result = literllm_default_client_chat(client, chat_completion_request_handle);
    literllm_chat_completion_response_free(result);
    literllm_chat_completion_request_free(chat_completion_request_handle);
    literllm_default_client_free(client);
    return EXIT_SUCCESS;
}

```
