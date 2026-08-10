---
id: fixture_php_embed_with_dimensions
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
$request = \Liter\Llm\EmbeddingRequest::from_json(json_encode(["dimensions" => 256, "input" => "Hello world", "model" => "text-embedding-3-small"]));
$result = LiterLlm::embed($request);

```
