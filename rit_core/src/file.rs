use std::io;
use std::io::{BufReader, Read};
use std::fs::File;
use std::string::String;
use std::path::Path;
use ring::digest;
use data_encoding::HEXUPPER;

pub fn sum256(path: &Path) -> io::Result<String> {
    let input = File::open(path)?;
    let mut reader = BufReader::new(input);

    let mut context = digest::Context::new(&digest::SHA256);
    let mut buffer = [0; 1024];
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }
    
    let digest_result = context.finish();
    let hex = HEXUPPER.encode(digest_result.as_ref());

    Ok(hex)
}