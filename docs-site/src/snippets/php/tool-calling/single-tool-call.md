---
id: fixture_php_single_tool_call
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
use Liter\Llm\Choice;
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["messages" => [["content" => "What is the weather in San Francisco?", "role" => "user"]], "model" => "gpt-4", "toolChoice" => "auto", "tools" => [["function" => ["description" => "Get the current weather for a given location", "name" => "get_weather", "parameters" => ["properties" => ["location" => ["description" => "The city and state, e.g. San Francisco, CA", "type" => "string"], "unit" => ["description" => "The temperature unit to use", "enum" => ["celsius", "fahrenheit"], "type" => "string"]], "required" => ["location"], "type" => "object"]], "type" => "function"]]]));
$result = LiterLlm::chat($request);

```
