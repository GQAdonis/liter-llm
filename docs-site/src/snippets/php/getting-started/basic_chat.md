---
id: readme_php_basic_chat
language: php
target: php
level: syntax
requires: []
side_effect: network
---

Send a message to any provider using the `provider/model` prefix.

```php
<?php

declare(strict_types=1);

use Liter\Llm\LiterLlm;
use Liter\Llm\ChatCompletionRequest;

$client = LiterLlm::createClient(getenv('OPENAI_API_KEY') ?: '');

$request = ChatCompletionRequest::from_json(json_encode([
    'model' => 'openai/gpt-4o-mini',
    'messages' => [['role' => 'user', 'content' => 'Hello!']],
]));

$result = $client->chat($request);
echo $result->choices[0]->message->content . PHP_EOL;
```
