---
id: fixture_php_bedrock_stream
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["maxTokens" => 32, "messages" => [["content" => "Count to three, one word per response.", "role" => "user"]], "model" => "bedrock/anthropic.claude-3-sonnet-20240229-v1:0", "stream" => true]));
$chunks = iterator_to_array(LiterLlm::chatStream($request));

```
