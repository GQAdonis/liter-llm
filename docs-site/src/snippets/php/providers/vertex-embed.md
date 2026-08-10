---
id: fixture_php_vertex_embed
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use Liter\Llm\LiterLlm;
use Liter\Llm\EmbeddingRequest;
$request = \Liter\Llm\EmbeddingRequest::from_json(json_encode(["input" => "Hello", "model" => "vertex_ai/text-embedding-005"]));
$result = LiterLlm::embed($request);

```
