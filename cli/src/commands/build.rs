use std::process::Command;

pub fn run() {
    println!("Building...");

    let status = Command::new("zig")
        .arg("build")
        .status()
        .unwrap_or_else(|_| {
            eprintln!("Error: 'zig' not found. Install zig and try again.");
            std::process::exit(1);
        });
    if status.success() {
        println!("Build succeeded!");
    } else {
        eprintln!("Build failed.");
        std::process::exit(1);
    }
}
