---
id: fixture_go_edge_batch_already_cancelled
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

func main() {
		client, clientErr := pkg.CreateClient("your-api-key", nil, nil, nil, nil)
	if clientErr != nil {
		panic(clientErr)
	}
	result, err := client.CancelBatch(`batch-cancelled001`)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
