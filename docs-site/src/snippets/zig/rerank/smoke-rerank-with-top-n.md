---
id: fixture_zig_smoke_rerank_with_top_n
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
    const _mock_url = try std.fmt.allocPrintSentinel(std.heap.c_allocator, "{s}/fixtures/smoke_rerank_with_top_n", .{if (std.c.getenv("MOCK_SERVER_URL")) |v| std.mem.span(v) else "http://localhost:8080"}, 0);
    var _client = try liter_llm.create_client("test-key", _mock_url, null, null, null);
    const _result_json = try _client.rerank("{\"documents\":[\"Python is a programming language.\",\"Cats are cute animals.\",\"Python was created by Guido van Rossum.\",\"The sun is a star.\"],\"model\":\"rerank-v3.5\",\"query\":\"What is Python?\",\"top_n\":2}");
}

```
