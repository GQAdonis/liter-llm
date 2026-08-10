---
id: fixture_php_edge_file_large_upload
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
$request = \Liter\Llm\CreateFileRequest::from_json(json_encode(["file" => "eyJwcm9tcHQiOiAibGFyZ2UgdHJhaW5pbmcgZGF0YSJ9Cg==", "filename" => "large_training_data.jsonl", "purpose" => "fine-tune"]));
$result = LiterLlm::createFile($request);

```
