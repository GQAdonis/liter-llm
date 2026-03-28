//go:build !liter_llm_dev
// +build !liter_llm_dev

//go:generate go run github.com/kreuzberg-dev/liter-llm/packages/go/cmd/install@latest

// Package literllm provides a Go client for the liter-llm universal LLM API.
//
// The go:generate directive above downloads the FFI library for your platform
// and generates the CGO flags needed to build. Run it once after installing:
//
//	go generate github.com/kreuzberg-dev/liter-llm/packages/go
//
// This eliminates the need to manually set CGO_CFLAGS and CGO_LDFLAGS environment variables.
package literllm
