---
id: fixture_php_smoke_cancel_batch
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use Liter\Llm\LiterLlm;
$result = LiterLlm::cancelBatch("batch-def456");

```
