---
id: fixture_go_error_response_not_found
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
	result, err := client.RetrieveResponse(`resp-nonexistent`)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
