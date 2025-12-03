const std = @import("std");

pub fn main() void {
    const puzzle_input = @embedFile("puzzle_input.txt");
    var iter = std.mem.splitScalar(u8, puzzle_input, '\n');

    var sum: usize = 0;

    while (iter.next()) |line| {
        const trimmed = std.mem.trim(u8, line, &std.ascii.whitespace);
        if (trimmed.len == 0) continue;
        sum += get_largest_pair(trimmed);
    }

    std.debug.print("Total pairs: {d}\n", .{sum});
}

pub fn get_largest_pair(input: []const u8) usize {
    if (input.len < 2) {
        return 0;
    }

    var max: usize = 0;

    for (input, 0..) |left, i| {
        const left_digit = std.fmt.charToDigit(left, 10) catch continue;
        for (input[i + 1 ..]) |right| {
            const right_digit = std.fmt.charToDigit(right, 10) catch continue;
            const pair_value: u8 = (left_digit * 10) + right_digit;
            if (pair_value > max) {
                max = pair_value;
            }
        }
    }

    return max;
}

test "known puzzle output" {
    const expect = @import("std").testing.expect;

    const puzzle_test = @embedFile("puzzle_input_test.txt");
    var iter = std.mem.splitScalar(u8, puzzle_test, '\n');

    var sum: usize = 0;

    while (iter.next()) |line| {
        const trimmed = std.mem.trim(u8, line, &std.ascii.whitespace);
        if (trimmed.len == 0) continue;
        sum += get_largest_pair(trimmed);
    }

    try expect(sum == 357);
}
