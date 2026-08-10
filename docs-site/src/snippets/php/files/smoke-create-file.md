---
id: fixture_php_smoke_create_file
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use Liter\Llm\LiterLlm;
use Liter\Llm\CreateFileRequest;
$request = \Liter\Llm\CreateFileRequest::from_json(json_encode(["file" => "eyJwcm9tcHQiOiAiaGVsbG8ifQo=", "filename" => "training_data.jsonl", "purpose" => "fine-tune"]));
$result = LiterLlm::createFile($request);

```
