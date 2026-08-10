---
id: fixture_php_edge_moderate_empty_input
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
$request = \Liter\Llm\ModerationRequest::from_json(json_encode(["model" => "omni-moderation-latest"]));
$result = LiterLlm::moderate($request);

```
