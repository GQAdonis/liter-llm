---
id: fixture_zig_multimodal_content
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
    const _mock_url = try std.fmt.allocPrintSentinel(std.heap.c_allocator, "{s}/fixtures/multimodal_content", .{if (std.c.getenv("MOCK_SERVER_URL")) |v| std.mem.span(v) else "http://localhost:8080"}, 0);
    var _client = try liter_llm.create_client("test-key", _mock_url, null, null, null);
    const _result_json = try _client.chat("{\"max_tokens\":100,\"messages\":[{\"content\":[{\"text\":\"What is in this image?\",\"type\":\"text\"},{\"image_url\":{\"detail\":\"low\",\"url\":\"https://upload.wikimedia.org/wikipedia/commons/thumb/4/47/PNG_transparency_demonstration_1.png/280px-PNG_transparency_demonstration_1.png\"},\"type\":\"image_url\"}],\"role\":\"user\"}],\"model\":\"gpt-4o\"}");
}

```
