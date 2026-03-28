//go:build !windows

package literllm

/*
// Liter-LLM FFI - CGO Configuration (non-Windows)
//
// This file provides the CGO include directive for the FFI header.
// Library linking is configured via:
//   - CI: CGO_CFLAGS and CGO_LDFLAGS environment variables
//   - Development: Use -tags liter_llm_dev for monorepo builds with hardcoded paths
//   - Production: Run go generate to download FFI and generate cgo_flags.go
//
// The CFLAGS directive below provides the include path for the header file.
// LDFLAGS must be provided externally (via env vars or cgo_flags.go).

#cgo CFLAGS: -I${SRCDIR}/internal/ffi

#include "internal/ffi/liter_llm.h"
#include <stdlib.h>
*/
import "C"
