use std::path::PathBuf;
use crate::chrome::error::ExporterError;
use crate::chrome::instance::ChromeInstance;

pub mod instance;
mod item;
mod user;
mod error;

pub fn export() -> Result<Vec<ChromeInstance>, ExporterError> {
    let mut output: Vec<ChromeInstance> = Vec::new();

    let local_dir: PathBuf = match dirs::cache_dir() {
        Some(pb) => pb,
        None => {
            return Err(ExporterError::CannotFindCache("Cannot find user's cache directory".to_string()));
        }
    };

    let chrome_dirs = vec![
        local_dir.join("Google/Chrome/User Data/"),
    ];

    for dir in chrome_dirs {
        if !dir.exists() {
            continue
        }

        let mut instance = ChromeInstance::new(dir)?;

        instance.load_users().expect("TODO: ERROR MESSAGE");

        output.push(instance);
    };

    Ok(output)
}
