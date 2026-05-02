pub const Mode = enum(u2) {
    mode0 = 0,
    mode1 = 1,
    mode2 = 2,
    mode3 = 3,
};

pub const Config = struct {
    mode: Mode = .mode0,
    max_freq_hz: u32 = 1_000_000,
    bit_order: enum { msb_first, lsb_first } = .msb_first,
};

pub fn Spi(comptime Impl: type) type {
    return struct {
        impl: Impl,

        const Self = @This();

        pub fn init(impl: Impl, config: Config) Self {
            impl.configure(config);
            return .{ .impl = impl };
        }

        pub fn transfer(self: Self, tx: []const u8, rx: []u8) !void {
            return self.impl.transfer(tx, rx);
        }

        pub fn write(self: Self, data: []const u8) !void {
            return self.impl.write(data);
        }
    };
}
