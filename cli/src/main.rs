use std::env;
use std::path::Path;
use std::io::Result;

use rit_core::repository::init_repository;
use rit_core::repository::status_repository;


fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args[1] == "init" {
            init_repository(Path::new("."))?;
            return Ok(())
        } else if args[1] == "status" {
            status_repository(Path::new("."))?;
            return Ok(())
        }
    }
    println!("rit --help");

    Ok(())
}
