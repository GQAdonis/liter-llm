---
id: fixture_java_contract_ocr
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
        var requestJson = "{\"document\":{\"type\":\"document_url\",\"url\":\"https://example.com/contract-test.pdf\"},\"model\":\"mistral/mistral-ocr-latest\"}";
var request = JsonUtil.fromJson(requestJson, OcrRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/contract_ocr";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.ocr(request);
    }
}

```
