---
id: fixture_php_ocr_multi_page
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
$request = \Liter\Llm\OcrRequest::from_json(json_encode(["document" => ["type" => "document_url", "url" => "https://example.com/multipage.pdf"], "model" => "mistral/mistral-ocr-latest"]));
$result = LiterLlm::ocr($request);

```
