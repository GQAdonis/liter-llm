---
id: fixture_php_smoke_create_batch
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
$result = LiterLlm::createBatch(new \Liter\Llm\CreateBatchRequest(inputFileId: "file-abc123",
endpoint: "/v1/chat/completions",
completionWindow: "24h"));

```
