```php
<?php

declare(strict_types=1);

use LiterLm\LlmClient;
use LiterLm\ChatRequest;
use LiterLm\Message;

$client = new LlmClient();
$response = $client->chat(new ChatRequest(
    model: 'openai/gpt-4o',
    messages: [
        new Message(role: 'user', content: 'Hello!'),
    ],
));
echo $response->getContent() . PHP_EOL;
```
