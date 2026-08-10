---
id: fixture_php_smoke_response_with_tools
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
$request = \Liter\Llm\CreateResponseRequest::from_json(json_encode(["input" => "What is the weather in San Francisco?", "model" => "gpt-4o", "tools" => [["description" => "Get current weather for a location", "name" => "get_weather", "parameters" => ["properties" => ["location" => ["type" => "string"]], "required" => ["location"], "type" => "object"], "type" => "function"]]]));
$result = LiterLlm::createResponse($request);

```
