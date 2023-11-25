use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn get_files_recursive(dir_path: PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut file_paths: Vec<PathBuf> = Vec::new();
    for entry in WalkDir::new(dir_path).follow_links(true) {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    if let Some(path) = entry.path().to_path_buf() {
                        file_paths.push(path);
                    }
                }
            }
            Err(e) => return Err(io::Error::from(e)),
        }
    }

    Ok(file_paths)
}