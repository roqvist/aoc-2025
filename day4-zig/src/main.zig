const std = @import("std");
const Map = @import("map").Map;
const TileType = @import("map").TileType;
const Tile = @import("map").Tile;
const ArrayList = std.ArrayList;

pub fn main() !void {
    const puzzle = @embedFile("puzzle_input.txt");
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer {
        const check = gpa.deinit();
        if (check == .leak) {
            std.debug.print("Memory leak detected!\n", .{});
        }
    }

    var map = try Map.init(puzzle, allocator);

    var accessable_roll_sum: usize = 0;

    var tile_iter = map.tiles.valueIterator();
    while (tile_iter.next()) |tile| {
        if (tile.tileType == TileType.Roll) {
            const all_neighbours = map.get_neighbours(tile);
            var nearby_rolls: usize = 0;
            for (all_neighbours) |maybe_neighbour| {
                if (maybe_neighbour) |neighbour| {
                    if (neighbour.tileType == TileType.Roll) {
                        nearby_rolls += 1;
                    }
                }
            }
            if (nearby_rolls < 4) {
                accessable_roll_sum += 1;
            }
        }
    }

    std.debug.print("Accessible roll count: {d}\n", .{accessable_roll_sum});
    map.deinit();

    map = try Map.init(puzzle, allocator);
    defer map.deinit();

    var removed_rolls: usize = 0;
    var rolls_were_removed = true;

    while (rolls_were_removed) {
        var tiles_to_remove: ArrayList(*Tile) = .empty;
        defer tiles_to_remove.deinit(allocator);

        var tile_iter2 = map.tiles.valueIterator();
        while (tile_iter2.next()) |tile| {
            if (tile.tileType == TileType.Roll) {
                const all_neighbours = map.get_neighbours(tile);
                var nearby_rolls: usize = 0;
                for (all_neighbours) |maybe_neighbour| {
                    if (maybe_neighbour) |neighbour| {
                        if (neighbour.tileType == TileType.Roll) {
                            nearby_rolls += 1;
                        }
                    }
                }
                if (nearby_rolls < 4) {
                    try tiles_to_remove.append(allocator, tile);
                    removed_rolls += 1;
                }
            }
        }

        if (tiles_to_remove.items.len > 0) {
            for (tiles_to_remove.items) |t| {
                map.remove_tile(t.pos.x, t.pos.y);
            }
            rolls_were_removed = true;
        } else {
            rolls_were_removed = false;
        }
    }

    std.debug.print("Total rolls removed: {d}\n", .{removed_rolls});
}

test "known puzzle output" {
    const expect = @import("std").testing.expect;
    const allocator = std.testing.allocator;

    const puzzle_test = @embedFile("puzzle_input_test.txt");
    var map = try Map.init(puzzle_test, allocator);
    defer map.deinit();

    var accessable_roll_sum: usize = 0;

    var tile_iter = map.tiles.valueIterator();
    while (tile_iter.next()) |tile| {
        if (tile.tileType == TileType.Roll) {
            const all_neighbours = map.get_neighbours(tile);
            var nearby_rolls: usize = 0;
            for (all_neighbours) |maybe_neighbour| {
                if (maybe_neighbour) |neighbour| {
                    if (neighbour.tileType == TileType.Roll) {
                        nearby_rolls += 1;
                    }
                }
            }
            if (nearby_rolls < 4) {
                accessable_roll_sum += 1;
            }
        }
    }

    try expect(accessable_roll_sum == 13);
}

test "known puzzle output part2" {
    const expect = @import("std").testing.expect;
    const allocator = std.testing.allocator;

    const puzzle_test = @embedFile("puzzle_input_test.txt");
    var map = try Map.init(puzzle_test, allocator);
    defer map.deinit();

    var removed_rolls: usize = 0;
    var rolls_were_removed = true;

    while (rolls_were_removed) {
        var tiles_to_remove: ArrayList(*Tile) = .empty;
        defer tiles_to_remove.deinit(allocator);

        var tile_iter = map.tiles.valueIterator();
        while (tile_iter.next()) |tile| {
            if (tile.tileType == TileType.Roll) {
                const all_neighbours = map.get_neighbours(tile);
                var nearby_rolls: usize = 0;
                for (all_neighbours) |maybe_neighbour| {
                    if (maybe_neighbour) |neighbour| {
                        if (neighbour.tileType == TileType.Roll) {
                            nearby_rolls += 1;
                        }
                    }
                }
                if (nearby_rolls < 4) {
                    try tiles_to_remove.append(allocator, tile);
                    removed_rolls += 1;
                }
            }
        }

        if (tiles_to_remove.items.len > 0) {
            for (tiles_to_remove.items) |t| {
                map.remove_tile(t.pos.x, t.pos.y);
            }
            rolls_were_removed = true;
        } else {
            rolls_were_removed = false;
        }
    }

    expect(removed_rolls == 43) catch |err| {
        std.debug.print("Removed rolls should be 43, but was {d}\n", .{removed_rolls});
        return err;
    };
}
