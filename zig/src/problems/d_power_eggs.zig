const std = @import("std");
const io = std.io;
const fmt = std.fmt;
const stdin = std.io.getStdIn().reader();
const stdout = io.getStdOut().writer();
// const stdin = std.io.getStdIn().inStream();
// const stdout = std.io.getStdOut().outStream();

const N = 33;

// Avoid alloc
var line_buf: [1024]u8 = undefined;

fn read_line() []u8 {
    return (stdin.readUntilDelimiterOrEof(&line_buf, '\n') catch unreachable) orelse "";
}

pub fn main() !void {
    const tc = try fmt.parseInt(u32, read_line(), 10);

    // dp table
    var dp: [N][N]u32 = undefined;
    var i: u32 = 0;
    var j: u32 = 0;
    // base case
    while (i < N) : (i += 1) {
        dp[0][i] = 0;
        dp[1][i] = i;
        dp[i][1] = 1;
    }
    i = 2;
    // fill the dp
    while (i < N) : (i += 1) {
        j = 2;
        while (j < N) : (j += 1) {
            dp[i][j] = dp[i - 1][j - 1] + dp[i][j - 1] + 1;
        }
    }

    // create for loop to read n lines
    i = 0;
    while (i < tc) : (i += 1) {
        const line = read_line();
        var iter = std.mem.split(u8, line, " ");
        var m = try fmt.parseInt(u32, iter.next().?, 10);
        var n = try fmt.parseInt(u32, iter.next().?, 10);
        if (dp[n][N - 1] < m) {
            try stdout.print("Impossible\n", .{});
            continue;
        }

        // j = 1;
        // while (j < N) : (j += 1) {
        //     if (dp[n][j] >= m) {
        //         try stdout.print("{d}\n", .{j});
        //         break;
        //     }
        // }
        var l: u8 = 1;
        var r: u8 = 32;
        while (l < r) {
            var mid: u8 = (l + r) / 2;

            if (dp[n][mid] >= m) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        try stdout.print("{d}\n", .{l});
    }
}
