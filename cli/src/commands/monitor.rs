use std::io::{self, BufRead, BufReader};

pub fn run(port: &str, baud: u32) {
    println!(
        "Connecting to {} at {} baud... (Ctrl+C to quit)",
        port, baud
    );

    let serial = serialport::new(port, baud).open().unwrap_or_else(|e| {
        eprintln!(
            "Error: could not open connection: {} - {}",
            port, e
        );
        std::process::exit(1);
    });

    let reader = BufReader::new(serial);

    for line in reader.lines() {
        match line {
            Ok(l) => println!("{}", l),
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => continue,
            Err(e) => {
                eprintln!("Read error: {}", e);
                break;
            }
        }
    }
}
