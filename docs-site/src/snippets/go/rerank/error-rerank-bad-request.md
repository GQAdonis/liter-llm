---
id: fixture_go_error_rerank_bad_request
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
	request := pkg.RerankRequest{
		Model: ptr(`nonexistent-rerank`),
		Query: ptr(`test`),
	}
		client, clientErr := pkg.CreateClient("your-api-key", nil, nil, nil, nil)
	if clientErr != nil {
		panic(clientErr)
	}
	result, err := client.Rerank(request)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
