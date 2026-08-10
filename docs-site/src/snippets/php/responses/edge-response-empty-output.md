---
id: fixture_php_edge_response_empty_output
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
$request = \Liter\Llm\CreateResponseRequest::from_json(json_encode(["model" => "gpt-4o"]));
$result = LiterLlm::createResponse($request);

```
