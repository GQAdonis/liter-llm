const std = @import("std");
const testing = std.testing;
const liter_llm = @import("liter_llm");

// `src/liter_llm.zig` is alef-generated and carries no `test` blocks of its own, ~keep
// so `zig build test` needs a hand-written entry point to have any coverage at ~keep
// all. This checks that the generated error set still declares the variant ~keep
// callers match on, catching an accidental rename/removal upstream. ~keep
test "LiterLlmError declares the NotFound variant" {
    const err: liter_llm.LiterLlmError = error.NotFound;
    try testing.expect(err == error.NotFound);
}
