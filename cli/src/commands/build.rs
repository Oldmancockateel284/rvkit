use std::process::Command;

pub fn run() {
    println!("Compilation en cours...");

    let status = Command::new("zig")
        .arg("build")
        .status()
        .unwrap_or_else(|_| {
            eprintln!("Erreur : 'zig' introuvable. Installe zig et reessaie.");
            std::process::exit(1);
        });
    if status.success() {
        println!("Build reussi !");
    } else {
        eprintln!("Build echoue");
        std::process::exit(1);
    }
}
