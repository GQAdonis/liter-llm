---
id: fixture_php_smoke_create_response
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use Liter\Llm\LiterLlm;
use Liter\Llm\CreateResponseRequest;
$request = \Liter\Llm\CreateResponseRequest::from_json(json_encode(["input" => "Explain quantum computing in one sentence.", "model" => "gpt-4o"]));
$result = LiterLlm::createResponse($request);

```
