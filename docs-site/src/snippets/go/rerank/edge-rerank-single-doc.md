---
id: fixture_go_edge_rerank_single_doc
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
		Model: ptr(`rerank-v3.5`),
		Query: ptr(`What is AI?`),
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
