use std::collections::HashMap;
use std::path::{Path, PathBuf};
use dirs;
use crate::chrome::item::ChromeItem;
use crate::chrome::user::ChromeUser;
use crate::error::ExporterError;
use crate::util::{get_files_recursive, read_local_state};


pub struct ChromeInstance {
    dir: PathBuf,
    master_key: Option<Vec<u8>>,
    users: HashMap<PathBuf, ChromeUser>,
    files: Vec<PathBuf>
}

impl ChromeInstance {
    pub fn new(dir: impl AsRef<Path>) -> Result<ChromeInstance, ExporterError> {
        let master_key: Option<Vec<u8>> = match read_local_state(dir.as_ref().join("Local State")) {
            Ok(key) => Some(key),
            Err(_) => {
                None
            }
        };

        let files: Vec<PathBuf> = get_files_recursive(dir.as_ref()).map_err(|e| {
            ExporterError::IO(e.to_string())
        })?;

        Ok(ChromeInstance {
            dir: dir.as_ref().to_path_buf(),
            master_key,
            users: HashMap::new(),
            files
        })
    }

    pub fn get_users(&self) -> Vec<ChromeUser> {
        self.users.clone().into_values().collect()
    }

    pub fn load_users(&mut self) -> Result<(), ExporterError> {
        for file in &self.files {
            if file.to_string_lossy().contains("System Profile") {
                continue
            }

            let chrome_item = match ChromeItem::new(file) {
                Ok(i) => i,
                Err(_) => {
                    continue;
                }
            };

            let mut profile_path = match file.parent() {
                Some(p) => p.to_path_buf(),
                None => {
                    continue
                },
            };

            // strips network cookies
            if file.ends_with(Path::new("Network/Cookies")) {
                if let Some(parent) = profile_path.parent() {
                    profile_path = parent.to_path_buf();
                }
            }

            self.users
                .entry(profile_path.clone())
                .or_insert_with(|| ChromeUser::new(self.master_key.clone()).unwrap())
                .add_item(chrome_item, file.clone());
        }

        Ok(())
    }
}


