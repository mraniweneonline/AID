use std::env;

use aid::{IdConfig, IdSystem, DEFAULT_MAX_LENGTH};

fn parse_length() -> Result<usize, String> {
    let mut args = env::args().skip(1);
    match args.next() {
        Some(value) => value
            .parse::<usize>()
            .map_err(|_| "length must be a positive integer".to_string()),
        None => Ok(DEFAULT_MAX_LENGTH),
    }
}

fn main() -> Result<(), String> {
    let length = parse_length()?;
    let system = IdSystem::new(IdConfig::default())?;
    let id = system.generate(length)?;
    println!("{id}");
    Ok(())
}
