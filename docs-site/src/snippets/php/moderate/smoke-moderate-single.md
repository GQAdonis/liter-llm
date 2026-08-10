---
id: fixture_php_smoke_moderate_single
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use Liter\Llm\LiterLlm;
use Liter\Llm\ModerationRequest;
$request = \Liter\Llm\ModerationRequest::from_json(json_encode(["input" => "The weather is nice today.", "model" => "omni-moderation-latest"]));
$result = LiterLlm::moderate($request);

```
