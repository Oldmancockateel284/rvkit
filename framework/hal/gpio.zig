pub const Direction = enum { input, output };
pub const Pull = enum { none, up, down };
pub const Level = enum(u1) { low = 0, high = 1 };

pub fn Gpio(comptime Impl: type) type {
    return struct {
        impl: Impl,

        const Self = @This();

        pub fn init(impl: Impl) Self {
            return .{ .impl = impl };
        }

        pub fn set_direction(self: Self, dir: Direction) void {
            self.impl.set_direction(dir);
        }

        pub fn set_pull(self: Self, pull: Pull) void {
            self.impl.set_pull(pull);
        }

        pub fn read(self: Self) Level {
            return self.impl.read();
        }

        pub fn write(self: Self, level: Level) void {
            self.impl.write(level);
        }

        pub fn toggle(self: Self) void {
            self.impl.toggle();
        }
    };
}
