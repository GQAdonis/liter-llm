---
id: fixture_php_stream_with_tool_calls
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["messages" => [["content" => "What is the weather in NYC?", "role" => "user"]], "model" => "gpt-4", "stream" => true, "tools" => [["function" => ["description" => "Get the current weather for a given location", "name" => "get_weather", "parameters" => ["properties" => ["location" => ["description" => "The city and state, e.g. New York, NY", "type" => "string"]], "required" => ["location"], "type" => "object"]], "type" => "function"]]]));
$chunks = iterator_to_array(LiterLlm::chatStream($request));

```
