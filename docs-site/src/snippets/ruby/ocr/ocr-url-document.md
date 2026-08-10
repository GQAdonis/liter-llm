---
id: fixture_ruby_ocr_url_document
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.ocr(LiterLlm::OcrRequest.new(document: { 'type' => 'document_url', 'url' => 'https://example.com/doc.pdf' }, model: 'mistral/mistral-ocr-latest'))

```
