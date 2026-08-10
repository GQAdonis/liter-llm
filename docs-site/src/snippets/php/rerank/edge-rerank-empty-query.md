---
id: fixture_php_edge_rerank_empty_query
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use Liter\Llm\LiterLlm;
use Liter\Llm\RerankRequest;
$request = \Liter\Llm\RerankRequest::from_json(json_encode(["documents" => ["Some document", "Another document"], "model" => "rerank-v3.5", "query" => ""]));
$result = LiterLlm::rerank($request);

```
