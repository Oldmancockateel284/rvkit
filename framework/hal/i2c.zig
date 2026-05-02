pub const Speed = enum { standard, fast, fast_plus };

pub const Config = struct {
    speed: Speed = .standard,
};

pub fn I2c(comptime Impl: type) type {
    return struct {
        impl: Impl,

        const Self = @This();

        pub fn init(impl: Impl, config: Config) Self {
            impl.configure(config);
            return .{ .impl = impl };
        }

        pub fn write(self: Self, addr: u7, data: []const u8) !void {
            return self.impl.write(addr, data);
        }

        pub fn read(self: Self, addr: u7, buf: []u8) !void {
            return self.impl.read(addr, buf);
        }

        pub fn write_read(
            self: Self,
            addr: u7,
            write_data: []const u8,
            read_buf: []u8,
        ) !void {
            return self.impl.write_read(addr, write_data, read_buf);
        }
    };
}
