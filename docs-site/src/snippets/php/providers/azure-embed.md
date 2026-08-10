---
id: fixture_php_azure_embed
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
$request = \Liter\Llm\EmbeddingRequest::from_json(json_encode(["input" => "Hello world", "model" => "azure/text-embedding-ada-002"]));
$result = LiterLlm::embed($request);

```
