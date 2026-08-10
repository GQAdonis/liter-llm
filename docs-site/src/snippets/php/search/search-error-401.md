---
id: fixture_php_search_error_401
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use Liter\Llm\LiterLlm;
use Liter\Llm\SearchRequest;
try {
    LiterLlm::search(new \Liter\Llm\SearchRequest(model: "brave/web-search",
query: "test"));
} catch (Throwable $error) {
    echo "Call failed as expected: {$error->getMessage()}\n";
    return;
}
throw new RuntimeException('expected call to fail');

```
