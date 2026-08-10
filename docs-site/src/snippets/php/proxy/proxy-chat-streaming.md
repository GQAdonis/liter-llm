---
id: fixture_php_proxy_chat_streaming
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["messages" => [["content" => "Count to 3", "role" => "user"]], "model" => "openai/gpt-4o", "stream" => true]));
$chunks = iterator_to_array(LiterLlm::chatStream($request));

```
