---
id: fixture_php_anthropic_tool_calling
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["maxTokens" => 256, "messages" => [["content" => "What is the weather in London?", "role" => "user"]], "model" => "anthropic/claude-3-5-sonnet-20241022", "toolChoice" => "auto", "tools" => [["function" => ["description" => "Get the current weather for a given location", "name" => "get_weather", "parameters" => ["properties" => ["location" => ["description" => "The city and country, e.g. London, UK", "type" => "string"], "unit" => ["description" => "The temperature unit to use", "enum" => ["celsius", "fahrenheit"], "type" => "string"]], "required" => ["location"], "type" => "object"]], "type" => "function"]]]));
$result = LiterLlm::chat($request);

```
