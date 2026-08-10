---
id: fixture_php_proxy_rerank
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
$request = \Liter\Llm\RerankRequest::from_json(json_encode(["documents" => ["Deep learning is a subset of machine learning using neural networks.", "The stock market closed higher today."], "model" => "rerank-v3.5", "query" => "What is deep learning?"]));
$result = LiterLlm::rerank($request);

```
