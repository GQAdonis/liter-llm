---
id: fixture_go_smoke_rerank_with_top_n
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
		Query: ptr(`What is Python?`),
		TopN:  2,
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
