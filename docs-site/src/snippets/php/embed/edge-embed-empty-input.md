---
id: fixture_php_edge_embed_empty_input
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
$request = \Liter\Llm\EmbeddingRequest::from_json(json_encode(["model" => "text-embedding-3-small"]));
$result = LiterLlm::embed($request);

```
