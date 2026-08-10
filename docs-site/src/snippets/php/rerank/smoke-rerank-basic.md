---
id: fixture_php_smoke_rerank_basic
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
$request = \Liter\Llm\RerankRequest::from_json(json_encode(["documents" => ["Machine learning is a subset of AI.", "The weather is sunny today.", "Deep learning uses neural networks."], "model" => "rerank-v3.5", "query" => "What is machine learning?"]));
$result = LiterLlm::rerank($request);

```
