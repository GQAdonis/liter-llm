---
id: fixture_php_response_format_json_schema
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use Liter\Llm\LiterLlm;
use Liter\Llm\ChatCompletionRequest;
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["messages" => [["content" => "What is the temperature in Paris today?", "role" => "user"]], "model" => "gpt-4", "responseFormat" => ["jsonSchema" => ["name" => "weather", "schema" => ["properties" => ["temp" => ["type" => "number"]], "required" => ["temp"], "type" => "object"]], "type" => "json_schema"]]));
$result = LiterLlm::chat($request);

```
