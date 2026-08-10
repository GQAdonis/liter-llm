---
id: fixture_php_embed_encoding_format
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
$request = \Liter\Llm\EmbeddingRequest::from_json(json_encode(["encodingFormat" => "float", "input" => "Test input", "model" => "text-embedding-3-small"]));
$result = LiterLlm::embed($request);

```
