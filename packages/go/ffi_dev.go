//go:build liter_llm_dev
// +build liter_llm_dev

package literllm

/*
// Liter-LLM FFI - Development Build Configuration
//
// This file provides LDFLAGS for development builds within the monorepo.
// It requires the "liter_llm_dev" build tag to be enabled:
//   go build -tags liter_llm_dev ./...
//   go test -tags liter_llm_dev ./...
//
// For production/external usage, run:
//   go generate github.com/kreuzberg-dev/liter-llm/packages/go
//
// This will download the FFI library and generate cgo_flags.go with
// the correct CGO directives for your platform.
//
// Build locations used:
//   Development: ${SRCDIR}/../../target/release/ (monorepo builds)

// macOS: Direct path to static library (Apple ld does not support -Bstatic)
#cgo darwin LDFLAGS: ${SRCDIR}/../../target/release/libliter_llm_ffi.a -framework CoreFoundation -framework CoreServices -framework SystemConfiguration -framework Security -lc++

// Linux: Use GNU ld static/dynamic switching
#cgo linux LDFLAGS: -L${SRCDIR}/../../target/release -Wl,-Bstatic -lliter_llm_ffi -Wl,-Bdynamic -lpthread -ldl -lm -lstdc++

// Windows: Static library with Windows system libs
#cgo windows LDFLAGS: -L${SRCDIR}/../../target/release -lliter_llm_ffi -lws2_32 -luserenv -lbcrypt -lntdll -static-libgcc -static-libstdc++
*/
import "C"
