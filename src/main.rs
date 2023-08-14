fn main() {
    let mut config = phen::Phen::parse().unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });

    config.run();
}
