const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer {
        const check = gpa.deinit();
        if (check == .leak) {
            std.debug.print("Memory leak detected!\n", .{});
        }
    }
    const puzzle = @embedFile("puzzle_input.txt");
    var maybe_ingredients = parse(puzzle, allocator);
    if (maybe_ingredients) |*ingredients| {
        defer ingredients.deinit();

        const unique_count = ingredients.countUniqueValues();
        std.debug.print("Unique values: {d}\n", .{unique_count});
    }
}

fn parse(input: []const u8, allocator: Allocator) ?RangeList {
    var parts = std.mem.splitSequence(u8, input, "\r\n\r\n");
    const ingredient_lines = parts.first();
    if (ingredient_lines.len < 3) {
        return null;
    }
    const fresh_lines = parts.rest();
    if (fresh_lines.len < 1) {
        return null;
    }

    var result = RangeList.init(allocator);

    var ingredient_ranges = std.mem.splitScalar(u8, ingredient_lines, '\n');
    while (ingredient_ranges.next()) |line| {
        const trimmed = std.mem.trim(u8, line, &std.ascii.whitespace);
        if (trimmed.len == 0) continue;
        var splitter = std.mem.splitScalar(u8, trimmed, '-');
        const start_range = splitter.first();
        const end_range = splitter.rest();
        const start = std.fmt.parseUnsigned(usize, start_range, 10) catch {
            continue;
        };
        const end = std.fmt.parseUnsigned(usize, end_range, 10) catch {
            continue;
        };
        result.add(IngredientRange{ .start = start, .end = end }, 0) catch {
            continue;
        };
    }

    result.finalize();

    var fresh_count: usize = 0;
    var total_ids: usize = 0;
    var freshness = std.mem.splitScalar(u8, fresh_lines, '\n');
    while (freshness.next()) |line| {
        const trimmed = std.mem.trim(u8, line, &std.ascii.whitespace);
        if (trimmed.len == 0) continue;
        const n = std.fmt.parseUnsigned(usize, trimmed, 10) catch {
            continue;
        };
        total_ids += 1;
        if (result.lookup(n) != null) {
            fresh_count += 1;
        }
    }

    std.debug.print("Total ranges: {}, Total IDs checked: {}, Fresh: {}\n", .{ result.ranges.items.len, total_ids, fresh_count });

    result.fresh_total = fresh_count;
    return result;
}
const RangeEntry = struct {
    range: IngredientRange,
    fresh: usize,
};

const RangeList = struct {
    ranges: ArrayList(RangeEntry),
    allocator: Allocator,
    fresh_total: usize,

    pub fn init(allocator: Allocator) RangeList {
        return .{ .ranges = ArrayList(RangeEntry){}, .allocator = allocator, .fresh_total = 0 };
    }

    pub fn deinit(self: *RangeList) void {
        self.ranges.deinit(self.allocator);
    }

    pub fn add(self: *RangeList, range: IngredientRange, fresh: usize) !void {
        try self.ranges.append(self.allocator, .{ .range = range, .fresh = fresh });
    }

    pub fn finalize(self: *RangeList) void {
        std.mem.sort(RangeEntry, self.ranges.items, {}, lessThan);
    }

    fn lessThan(_: void, a: RangeEntry, b: RangeEntry) bool {
        return a.range.start < b.range.start;
    }

    pub fn countUniqueValues(self: *const RangeList) usize {
        if (self.ranges.items.len == 0) return 0;

        var total: usize = 0;
        var current_start = self.ranges.items[0].range.start;
        var current_end = self.ranges.items[0].range.end;

        for (self.ranges.items[1..]) |entry| {
            if (entry.range.start <= current_end + 1) {
                current_end = @max(current_end, entry.range.end);
            } else {
                total += (current_end - current_start + 1);
                current_start = entry.range.start;
                current_end = entry.range.end;
            }
        }

        total += (current_end - current_start + 1);
        return total;
    }

    pub fn lookup(self: *const RangeList, value: usize) ?usize {
        for (self.ranges.items) |entry| {
            if (entry.range.contains(value)) {
                return entry.fresh;
            }
        }
        return null;
    }
};

const IngredientRange = struct {
    start: usize,
    end: usize,

    pub fn contains(self: IngredientRange, value: usize) bool {
        return value >= self.start and value <= self.end;
    }
};

test "known puzzle output" {
    const expect = @import("std").testing.expect;
    const allocator = std.testing.allocator;
    const puzzle = @embedFile("puzzle_input_test.txt");
    var maybe_ingredients = parse(puzzle, allocator);
    if (maybe_ingredients) |*ingredients| {
        defer ingredients.deinit();
        try expect(ingredients.ranges.items.len > 0);
        try expect(ingredients.fresh_total == 3);
    }
}

test "known puzzle output 2" {
    const expect = @import("std").testing.expect;
    const allocator = std.testing.allocator;
    const puzzle = @embedFile("puzzle_input_test.txt");
    var maybe_ingredients = parse(puzzle, allocator);
    if (maybe_ingredients) |*ingredients| {
        defer ingredients.deinit();

        const unique_count = ingredients.countUniqueValues();
        std.debug.print("Unique values: {d}\n", .{unique_count});
        try expect(unique_count == 14);
    }
}
