---
id: fixture_c_anthropic_tool_calling
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
    LITERLLMChatCompletionRequest* chat_completion_request_handle = literllm_chat_completion_request_from_json("{\"max_tokens\":256,\"messages\":[{\"content\":\"What is the weather in London?\",\"role\":\"user\"}],\"model\":\"anthropic/claude-3-5-sonnet-20241022\",\"tool_choice\":\"auto\",\"tools\":[{\"function\":{\"description\":\"Get the current weather for a given location\",\"name\":\"get_weather\",\"parameters\":{\"properties\":{\"location\":{\"description\":\"The city and country, e.g. London, UK\",\"type\":\"string\"},\"unit\":{\"description\":\"The temperature unit to use\",\"enum\":[\"celsius\",\"fahrenheit\"],\"type\":\"string\"}},\"required\":[\"location\"],\"type\":\"object\"}},\"type\":\"function\"}]}");
    LITERLLMDefaultClient* client = literllm_create_client("test-key", NULL, (uint64_t)-1, (uint32_t)-1, NULL);
    LITERLLMChatCompletionResponse* result = literllm_default_client_chat(client, chat_completion_request_handle);
    literllm_chat_completion_response_free(result);
    literllm_chat_completion_request_free(chat_completion_request_handle);
    literllm_default_client_free(client);
    return EXIT_SUCCESS;
}

```
