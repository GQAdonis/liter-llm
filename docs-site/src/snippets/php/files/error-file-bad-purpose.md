---
id: fixture_php_error_file_bad_purpose
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use Liter\Llm\LiterLlm;
use Liter\Llm\CreateFileRequest;
$request = \Liter\Llm\CreateFileRequest::from_json(json_encode(["file" => "data.jsonl", "purpose" => "invalid-purpose"]));
try {
    LiterLlm::createFile($request);
} catch (Throwable $error) {
    echo "Call failed as expected: {$error->getMessage()}\n";
    return;
}
throw new RuntimeException('expected call to fail');

```
