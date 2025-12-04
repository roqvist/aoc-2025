const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const AutoHashMap = std.AutoHashMap;

pub const Map = struct {
    tiles: AutoHashMap(Point2D, Tile),
    allocator: Allocator,

    pub fn init(input: []const u8, allocator: Allocator) !Map {
        var iter = std.mem.splitScalar(u8, input, '\n');
        var tiles = AutoHashMap(Point2D, Tile).init(allocator);

        var y: usize = 0;
        while (iter.next()) |line| : (y += 1) {
            const trimmed = std.mem.trim(u8, line, &std.ascii.whitespace);
            if (trimmed.len == 0) continue;
            for (line, 0..) |c, x| {
                const pos = Point2D{ .x = x, .y = y };
                if (c == '.') {
                    try tiles.put(pos, Tile{ .tileType = TileType.Empty, .pos = pos });
                } else if (c == '@') {
                    try tiles.put(pos, Tile{ .tileType = TileType.Roll, .pos = pos });
                } else {
                    continue;
                }
            }
        }

        std.debug.print("New map initialized with {d} tiles\n", .{tiles.count()});
        return Map{ .tiles = tiles, .allocator = allocator };
    }

    pub fn remove_tile(self: *Map, x: usize, y: usize) void {
        _ = self.tiles.remove(Point2D{ .x = x, .y = y });
    }

    pub fn get_tile(self: *Map, x: usize, y: usize) ?*Tile {
        return self.tiles.getPtr(Point2D{ .x = x, .y = y });
    }

    pub fn get_neighbours(self: *Map, tile: *const Tile) [8]?*Tile {
        var result = [_]?*Tile{null} ** 8;
        var index: usize = 0;

        const offsets = [_][2]i32{
            .{ -1, -1 }, .{ 0, -1 }, .{ 1, -1 },
            .{ -1, 0 },  .{ 1, 0 },  .{ -1, 1 },
            .{ 0, 1 },   .{ 1, 1 },
        };

        for (offsets) |offset| {
            const new_x = @as(i32, @intCast(tile.pos.x)) + offset[0];
            const new_y = @as(i32, @intCast(tile.pos.y)) + offset[1];

            if (new_x < 0 or new_y < 0) continue;

            const target_x = @as(usize, @intCast(new_x));
            const target_y = @as(usize, @intCast(new_y));

            if (self.tiles.getPtr(Point2D{ .x = target_x, .y = target_y })) |t| {
                result[index] = t;
                index += 1;
            }
        }

        return result;
    }

    pub fn deinit(self: *Map) void {
        self.tiles.deinit();
    }
};

pub const Tile = struct {
    tileType: TileType,
    pos: Point2D,
};

const Point2D = struct { x: usize, y: usize };

pub const TileType = enum { Empty, Roll };
