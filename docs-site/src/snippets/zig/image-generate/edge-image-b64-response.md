---
id: fixture_zig_edge_image_b64_response
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
    const _mock_url = try std.fmt.allocPrintSentinel(std.heap.c_allocator, "{s}/fixtures/edge_image_b64_response", .{if (std.c.getenv("MOCK_SERVER_URL")) |v| std.mem.span(v) else "http://localhost:8080"}, 0);
    var _client = try liter_llm.create_client("test-key", _mock_url, null, null, null);
    const _result_json = try _client.image_generate("{\"model\":\"dall-e-3\",\"n\":1,\"prompt\":\"A blue circle\",\"response_format\":\"b64_json\",\"size\":\"1024x1024\"}");
}

```
