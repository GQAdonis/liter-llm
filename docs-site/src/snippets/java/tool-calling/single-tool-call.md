---
id: fixture_java_single_tool_call
language: java
target: java
level: typecheck
requires: []
side_effect: safe
---

```java title="Java"
import io.xberg.literllm.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var requestJson = "{\"messages\":[{\"content\":\"What is the weather in San Francisco?\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"tool_choice\":\"auto\",\"tools\":[{\"function\":{\"description\":\"Get the current weather for a given location\",\"name\":\"get_weather\",\"parameters\":{\"properties\":{\"location\":{\"description\":\"The city and state, e.g. San Francisco, CA\",\"type\":\"string\"},\"unit\":{\"description\":\"The temperature unit to use\",\"enum\":[\"celsius\",\"fahrenheit\"],\"type\":\"string\"}},\"required\":[\"location\"],\"type\":\"object\"}},\"type\":\"function\"}]}";
var request = JsonUtil.fromJson(requestJson, ChatCompletionRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/single_tool_call";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.chat(request);
    }
}

```
