---
id: fixture_ruby_ocr_multi_page
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.ocr(LiterLlm::OcrRequest.new(document: { 'type' => 'document_url', 'url' => 'https://example.com/multipage.pdf' }, model: 'mistral/mistral-ocr-latest'))

```
