---
id: fixture_php_error_response_auth_401
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use Liter\Llm\LiterLlm;
use Liter\Llm\CreateResponseRequest;
$request = \Liter\Llm\CreateResponseRequest::from_json(json_encode(["input" => "Hello", "model" => "gpt-4o"]));
try {
    LiterLlm::createResponse($request);
} catch (Throwable $error) {
    echo "Call failed as expected: {$error->getMessage()}\n";
    return;
}
throw new RuntimeException('expected call to fail');

```
