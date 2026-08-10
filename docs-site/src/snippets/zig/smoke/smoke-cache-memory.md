---
id: fixture_zig_smoke_cache_memory
language: zig
target: zig
level: typecheck
requires: []
side_effect: safe
---

```zig title="Zig"
const std = @import("std");
const liter_llm = @import("liter_llm");

pub fn main() !void {
    const _mock_url = try std.fmt.allocPrintSentinel(std.heap.c_allocator, "{s}/fixtures/smoke_cache_memory", .{if (std.c.getenv("MOCK_SERVER_URL")) |v| std.mem.span(v) else "http://localhost:8080"}, 0);
    var _client = try liter_llm.create_client("test-key", _mock_url, null, null, null);
    const _result_json = try _client.chat("{\"max_tokens\":5,\"messages\":[{\"content\":\"What is 2+2? Answer with just the number.\",\"role\":\"user\"}],\"model\":\"openai/gpt-4o-mini\"}");
}

```
