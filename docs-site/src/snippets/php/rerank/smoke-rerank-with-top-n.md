---
id: fixture_php_smoke_rerank_with_top_n
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
$request = \Liter\Llm\RerankRequest::from_json(json_encode(["documents" => ["Python is a programming language.", "Cats are cute animals.", "Python was created by Guido van Rossum.", "The sun is a star."], "model" => "rerank-v3.5", "query" => "What is Python?", "topN" => 2]));
$result = LiterLlm::rerank($request);

```
