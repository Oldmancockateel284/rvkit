use std::io::{self, BufRead, BufReader};

pub fn run(port: &str, baud: u32) {
    println!(
        "Connexion sur {} à {} bauds... (Ctrl + C pour quitter)",
        port, baud
    );

    let serial = serialport::new(port, baud).open().unwrap_or_else(|e| {
        eprintln!(
            "Erreur : impossible d'ouvrir la connexion : {} - {}",
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
                eprintln!("Erreur de lecture : {}", e);
                break;
            }
        }
    }
}
