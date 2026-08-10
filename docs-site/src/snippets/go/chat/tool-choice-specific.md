---
id: fixture_go_tool_choice_specific
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
	request := pkg.ChatCompletionRequest{
		Model:      ptr(`gpt-4`),
		ToolChoice: ptr(pkg.ToolChoice(`{"function":{"name":"get_weather"},"type":"function"}`)),
	}
		client, clientErr := pkg.CreateClient("your-api-key", nil, nil, nil, nil)
	if clientErr != nil {
		panic(clientErr)
	}
	result, err := client.Chat(request)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
