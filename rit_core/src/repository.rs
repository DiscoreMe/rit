use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use walkdir::WalkDir;

use crate::file::sum256;

pub fn init_repository(path: &Path) -> io::Result<()> {
    let dir_path: PathBuf = path.join(".rit");
    fs::create_dir(dir_path)?;
    
    let file_path: PathBuf = path.join(".rit").join("set");
    match fs::OpenOptions::new().create(true).write(true).open(file_path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
pub fn status_repository(path: &Path) -> io::Result<()> {
    for entry in WalkDir::new(path) {
        let entry: walkdir::DirEntry = entry?;
        let path = entry.path();
        let filename = entry.file_name();
        let metadata: fs::Metadata = entry.metadata()?;
        
        if metadata.is_file() {
            println!("filename: {:?} | hash: {:?}", filename, sum256(path)?);
        }
    }

    Ok(())
}