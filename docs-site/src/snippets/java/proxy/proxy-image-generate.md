---
id: fixture_java_proxy_image_generate
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
        var requestJson = "{\"model\":\"dall-e-3\",\"n\":1,\"prompt\":\"A sunset over the ocean\",\"size\":\"1024x1024\"}";
var request = JsonUtil.fromJson(requestJson, CreateImageRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/proxy_image_generate";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.imageGenerate(request);
    }
}

```
