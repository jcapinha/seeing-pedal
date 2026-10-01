//! Shared laptop-host plumbing for `host-wsl` and `host-windows`.

use std::error::Error;

/// Stub entry point until audio I/O is wired up.
pub fn run(host_name: &str) -> Result<(), Box<dyn Error>> {
    println!("{host_name}: stub (engine = {engine})", engine = engine::ENGINE_STUB);
    Ok(())
}
