use std::{fs, io};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn get_files_recursive(dir_path: impl AsRef<Path>) -> io::Result<Vec<PathBuf>> {
    let mut file_paths: Vec<PathBuf> = Vec::new();
    for entry in WalkDir::new(dir_path).follow_links(true) {
        let entry = entry?;
        if entry.file_type().is_file() {
            file_paths.push(entry.path().to_path_buf());
        }
    }

    Ok(file_paths)
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in WalkDir::new(src).follow_links(true) {
        let entry = entry?;
        if entry.file_type().is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}