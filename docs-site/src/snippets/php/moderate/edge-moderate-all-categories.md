---
id: fixture_php_edge_moderate_all_categories
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
$request = \Liter\Llm\ModerationRequest::from_json(json_encode(["input" => "Extremely harmful content targeting multiple categories", "model" => "omni-moderation-latest"]));
$result = LiterLlm::moderate($request);

```
