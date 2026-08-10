---
id: fixture_php_tool_choice_specific
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["messages" => [["content" => "What is the weather in Paris?", "role" => "user"]], "model" => "gpt-4", "toolChoice" => ["function" => ["name" => "get_weather"], "type" => "function"], "tools" => [["function" => ["description" => "Get the current weather for a given location", "name" => "get_weather", "parameters" => ["properties" => ["location" => ["description" => "The city name", "type" => "string"]], "required" => ["location"], "type" => "object"]], "type" => "function"], ["function" => ["description" => "Search the web for information", "name" => "search_web", "parameters" => ["properties" => ["query" => ["description" => "The search query", "type" => "string"]], "required" => ["query"], "type" => "object"]], "type" => "function"]]]));
$result = LiterLlm::chat($request);

```
