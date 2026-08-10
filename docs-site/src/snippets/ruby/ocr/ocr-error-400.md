---
id: fixture_ruby_ocr_error_400
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.ocr(LiterLlm::OcrRequest.new(document: { 'type' => 'document_url', 'url' => 'invalid://url' }, model: 'mistral/mistral-ocr-latest'))

```
