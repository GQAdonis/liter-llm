---
id: fixture_java_ocr_multi_page
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
        var requestJson = "{\"document\":{\"type\":\"document_url\",\"url\":\"https://example.com/multipage.pdf\"},\"model\":\"mistral/mistral-ocr-latest\"}";
var request = JsonUtil.fromJson(requestJson, OcrRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/ocr_multi_page";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.ocr(request);
    }
}

```
