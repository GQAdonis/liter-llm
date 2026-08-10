---
id: fixture_php_all_message_types
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["messages" => [["content" => "You are a helpful assistant.", "role" => "system"], ["content" => "What is the weather in Paris?", "role" => "user"], ["content" => null, "role" => "assistant", "toolCalls" => [["function" => ["arguments" => "{\"location\": \"Paris, France\"}", "name" => "get_weather"], "id" => "call_xyz789", "type" => "function"]]], ["content" => "{\"temperature\": 18, \"unit\": \"celsius\", \"description\": \"Partly cloudy\"}", "role" => "tool", "toolCallId" => "call_xyz789"]], "model" => "gpt-4"]));
$result = LiterLlm::chat($request);

```
