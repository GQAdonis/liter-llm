---
id: fixture_php_embed_base64
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
$request = \Liter\Llm\EmbeddingRequest::from_json(json_encode(["encodingFormat" => "base64", "input" => "Test input", "model" => "text-embedding-3-small"]));
$result = LiterLlm::embed($request);

```
