---
id: fixture_go_smoke_retrieve_response
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
	result, err := client.RetrieveResponse(`resp-abc123`)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
