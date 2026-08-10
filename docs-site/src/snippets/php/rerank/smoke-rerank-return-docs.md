---
id: fixture_php_smoke_rerank_return_docs
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
$request = \Liter\Llm\RerankRequest::from_json(json_encode(["documents" => ["Rust is a systems programming language.", "Iron rusts when exposed to water."], "model" => "rerank-v3.5", "query" => "What is Rust?", "returnDocuments" => true]));
$result = LiterLlm::rerank($request);

```
