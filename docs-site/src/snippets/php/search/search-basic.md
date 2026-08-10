---
id: fixture_php_search_basic
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
query: "What is Rust programming language?"));

```
