---
id: fixture_php_anthropic_stream
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["maxTokens" => 32, "messages" => [["content" => "Count to three, one word per response.", "role" => "user"]], "model" => "anthropic/claude-3-5-sonnet-20241022", "stream" => true]));
$chunks = iterator_to_array(LiterLlm::chatStream($request));

```
