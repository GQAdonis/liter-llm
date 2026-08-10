---
id: fixture_zig_smoke_rerank_return_docs
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
    const _mock_url = try std.fmt.allocPrintSentinel(std.heap.c_allocator, "{s}/fixtures/smoke_rerank_return_docs", .{if (std.c.getenv("MOCK_SERVER_URL")) |v| std.mem.span(v) else "http://localhost:8080"}, 0);
    var _client = try liter_llm.create_client("test-key", _mock_url, null, null, null);
    const _result_json = try _client.rerank("{\"documents\":[\"Rust is a systems programming language.\",\"Iron rusts when exposed to water.\"],\"model\":\"rerank-v3.5\",\"query\":\"What is Rust?\",\"return_documents\":true}");
}

```
