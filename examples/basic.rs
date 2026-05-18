fn main() {
    match test_configuration_loader::config::load() {
        Ok(cfg) => println!("Loaded config: {:#?}", cfg),
        Err(e) => eprintln!("Failed to load config: {}", e),
    }
}
