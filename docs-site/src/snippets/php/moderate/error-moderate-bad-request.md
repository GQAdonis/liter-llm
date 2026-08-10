---
id: fixture_php_error_moderate_bad_request
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use Liter\Llm\LiterLlm;
use Liter\Llm\ModerationRequest;
$request = \Liter\Llm\ModerationRequest::from_json(json_encode(["input" => "Hello", "model" => "nonexistent-moderation"]));
try {
    LiterLlm::moderate($request);
} catch (Throwable $error) {
    echo "Call failed as expected: {$error->getMessage()}\n";
    return;
}
throw new RuntimeException('expected call to fail');

```
