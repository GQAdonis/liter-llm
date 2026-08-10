---
id: fixture_php_error_batch_auth_401
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use Liter\Llm\LiterLlm;
use Liter\Llm\CreateBatchRequest;
try {
    LiterLlm::createBatch(new \Liter\Llm\CreateBatchRequest(inputFileId: "file-abc123",
endpoint: "/v1/chat/completions",
completionWindow: "24h"));
} catch (Throwable $error) {
    echo "Call failed as expected: {$error->getMessage()}\n";
    return;
}
throw new RuntimeException('expected call to fail');

```
