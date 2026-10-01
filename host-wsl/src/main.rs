use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    host_common::run("seeing-pedal host-wsl")
}
