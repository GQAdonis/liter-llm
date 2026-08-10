---
id: fixture_go_ocr_url_document
language: go
target: go
level: typecheck
requires: []
side_effect: safe
---

```go title="Go"
package main

import (
	"fmt"
	pkg "github.com/xberg-io/liter-llm/packages/go"
)

func ptr[T any](value T) *T { return &value }
func main() {
	request := pkg.OcrRequest{
		Model:    ptr(`mistral/mistral-ocr-latest`),
		Document: ptr(pkg.OcrDocument(`{"type":"document_url","url":"https://example.com/doc.pdf"}`)),
	}
		client, clientErr := pkg.CreateClient("your-api-key", nil, nil, nil, nil)
	if clientErr != nil {
		panic(clientErr)
	}
	result, err := client.Ocr(request)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
