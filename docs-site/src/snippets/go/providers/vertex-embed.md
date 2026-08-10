---
id: fixture_go_vertex_embed
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
	request := pkg.EmbeddingRequest{
		Model: ptr(`vertex_ai/text-embedding-005`),
		Input: ptr(pkg.EmbeddingInput(`Hello`)),
	}
		client, clientErr := pkg.CreateClient("your-api-key", nil, nil, nil, nil)
	if clientErr != nil {
		panic(clientErr)
	}
	result, err := client.Embed(request)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
