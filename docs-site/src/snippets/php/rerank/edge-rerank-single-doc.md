---
id: fixture_php_edge_rerank_single_doc
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
$request = \Liter\Llm\RerankRequest::from_json(json_encode(["documents" => ["Artificial intelligence is the simulation of human intelligence."], "model" => "rerank-v3.5", "query" => "What is AI?"]));
$result = LiterLlm::rerank($request);

```
