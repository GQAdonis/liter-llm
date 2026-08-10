---
id: fixture_php_contract_search
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
$result = LiterLlm::search(new \Liter\Llm\SearchRequest(model: "brave/web-search",
query: "contract test query"));

```
