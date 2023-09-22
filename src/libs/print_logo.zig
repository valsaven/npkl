const print = @import("std").debug.print;

pub fn printLogo() void {
    print("{s}\n", .{
        \\              _    _
        \\             | |  | |
        \\  _ __  _ __ | | _| |
        \\ | '_ \| '_ \| |/ / |
        \\ | | | | |_) |   <| |
        \\ |_| |_| .__/|_|\_\_|
        \\       |_|
    });
}
