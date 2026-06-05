const std = @import("std");

// A plain enum. The tag values are assigned automatically (pending == 0).
const Status = enum { pending, running, succeeded, failed };

// Zig has first-class tagged unions: `union(enum)` pairs each variant with a
// type, and the compiler stores the active tag for you. Unlike C, you cannot
// read the wrong member, and `switch` must cover every variant.
const Media = union(enum) {
    text: struct { length: u32 },
    image: struct { width: u32, height: u32 },
    video: struct { width: u32, height: u32, duration: f64 },
};

fn describe(m: Media) void {
    switch (m) {
        .text => |t| std.debug.print("text: {d} chars\n", .{t.length}),
        .image => |i| std.debug.print("image: {d}x{d}\n", .{ i.width, i.height }),
        .video => |v| std.debug.print("video: {d}x{d}, {d}s\n", .{ v.width, v.height, v.duration }),
    }
}

pub fn main() void {
    const s: Status = .running;
    std.debug.print("status: {s} ({d})\n", .{ @tagName(s), @intFromEnum(s) });

    const items = [_]Media{
        .{ .text = .{ .length = 280 } },
        .{ .image = .{ .width = 1920, .height = 1080 } },
        .{ .video = .{ .width = 1920, .height = 1080, .duration = 12.5 } },
    };
    for (items) |m| describe(m);
}
