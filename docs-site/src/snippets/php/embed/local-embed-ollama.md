---
id: fixture_php_local_embed_ollama
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
$request = \Liter\Llm\EmbeddingRequest::from_json(json_encode(["input" => "The quick brown fox jumps over the lazy dog", "model" => "ollama/all-minilm"]));
$result = LiterLlm::embed($request);

```
