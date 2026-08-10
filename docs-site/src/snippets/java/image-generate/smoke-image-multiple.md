---
id: fixture_java_smoke_image_multiple
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
        var requestJson = "{\"model\":\"dall-e-2\",\"n\":3,\"prompt\":\"A red bicycle\",\"size\":\"256x256\"}";
var request = JsonUtil.fromJson(requestJson, CreateImageRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/smoke_image_multiple";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.imageGenerate(request);
    }
}

```
