pub const Config = struct {
    baud_rate: u32 = 115200,
    data_bits: enum { seven, eight } = .eight,
    stop_bits: enum { one, two } = .one,
    parity: enum { none, even, odd } = .none,
};

pub fn Uart(comptime Impl: type) type {
    return struct {
        impl: Impl,

        const Self = @This();

        pub fn init(impl: Impl, config: Config) Self {
            impl.configure(config);
            return .{ .impl = impl };
        }

        pub fn write_byte(self: Self, byte: u8) void {
            self.impl.write_byte(byte);
        }

        pub fn write(self: Self, data: []const u8) void {
            for (data) |byte| {
                self.impl.write_byte(byte);
            }
        }

        pub fn read_byte(self: Self) ?u8 {
            return self.impl.read_byte();
        }
    };
}
