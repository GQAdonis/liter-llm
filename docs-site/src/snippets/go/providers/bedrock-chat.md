---
id: fixture_go_bedrock_chat
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
		Model:       ptr(`bedrock/anthropic.claude-3-sonnet-20240229-v1:0`),
		Temperature: 0,
		MaxTokens:   16,
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
