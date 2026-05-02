pub struct Board {
    pub name: &'static str,
    pub cpu_arch: &'static str,
    pub linker_script: &'static str,
    pub flash_tool: &'static str,
}

pub const CH32V003: Board = Board {
    name: "ch32v003",
    cpu_arch: "riscv32",
    linker_script: include_str!("../../framework/linker/ch322v003.ld"),
    flash_tool: "wlink",
};

pub const ESP32_C3: Board = Board {
    name: "esp32-c3",
    cpu_arch: "riscv32",
    linker_script: include_str!("../../framework/linker/esp32c3.ld"),
    flash_tool: "esptool",
};

pub fn get(name: &str) -> Option<&'static Board> {
    match name {
        "ch32v003" => Some(&CH32V003),
        "esp32-c3" => Some(&ESP32_C3),
        _ => None,
    }
}


pub fn list() -> &'static [&'static Board] {
    &[&CH32V003, &ESP32_C3]
}
