const std = @import("std");
const print = std.debug.print;

const printLogo = @import("./libs/print_logo.zig").printLogo;
const isTopLevelNodeModules = @import("./libs/is_top_level_node_modules.zig").isTopLevelNodeModules;

pub fn main() !void {
    // Prints to stderr (it's a shortcut based on `std.io.getStdErr()`)
    printLogo();

    const path = "/lalala/dev/test/123";

    // const isTopLevel = try isTopLevelNodeModules(path) catch print("Отсутствует папка node_modules.", {});
    // _ = isTopLevel;

    const result = isTopLevelNodeModules(path) catch {
        std.debug.print("Word not found.\n", .{});
        return;
    };

    switch (result) {
        true => std.debug.print("One match found.\n", .{}),
        false => std.debug.print("More than one match found.\n", .{}),
    }

    // switch (try isTopLevelNodeModules(path)) {
    //     true => std.debug.print("One match found.\n", .{}),
    //     false => std.debug.print("More than one match found.\n", .{}),
    //     null => unreachable, // will never get here
    //     else => std.debug.print("Word not found.\n", .{}),
    // }

    // switch (try isTopLevelNodeModules(path)) {
    //     true => std.debug.print("One match found.\n", .{}),
    //     false => std.debug.print("More than one match found.\n", .{}),
    // }

    // stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    try stdout.print("Run `zig build test` to run the tests.\n", .{});

    try bw.flush(); // don't forget to flush!
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}
