---
id: fixture_php_ocr_error_401
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use Liter\Llm\LiterLlm;
use Liter\Llm\OcrRequest;
$request = \Liter\Llm\OcrRequest::from_json(json_encode(["document" => ["type" => "document_url", "url" => "https://example.com/doc.pdf"], "model" => "mistral/mistral-ocr-latest"]));
try {
    LiterLlm::ocr($request);
} catch (Throwable $error) {
    echo "Call failed as expected: {$error->getMessage()}\n";
    return;
}
throw new RuntimeException('expected call to fail');

```
