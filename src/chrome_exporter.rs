use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use dirs;
use crate::util::{get_files_recursive, read_local_state};
use crate::chrome_item::ChromeItem;
use crate::chrome_profile::ChromeProfile;



struct ChromeExporter {
    dir: PathBuf,
    local_state: PathBuf,
    master_key: Vec<u8>,
    users: HashMap<PathBuf, ChromeProfile>,
    files: Vec<PathBuf>,
}

impl ChromeExporter {
    pub fn new(dir: impl AsRef<Path>) -> Result<ChromeExporter, String> {
        let master_key = match read_local_state(dir.as_ref().join("Local State")) {
            Ok(key) => key,
            Err(e) => return Err(e),
        };

        Ok(ChromeExporter {
            dir: dir.as_ref().to_path_buf(),
            local_state: dir.as_ref().join("Local State"),
            master_key: master_key,
            users: HashMap::new(),
            files: Vec::new(),
        })
    }

    fn get_files(&mut self) -> io::Result<()> {
        self.files = get_files_recursive(&self.dir)?;
        Ok(())
    }

    fn export(&mut self) -> Result<(), String> {
        for (path, profile) in &self.users {
            profile.read_login_data()?;
            profile.read_cookies()?;
        };

        Ok(())
    }

    pub fn load_files(&mut self) -> Result<(), String> {
        match self.get_files() {
            Ok(_) => {},
            Err(_) => {
                return Err("Failed to load files".to_string())
            }
        }

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
                .or_insert_with(|| ChromeProfile::new(self.master_key.clone()).unwrap())
                .add_item(chrome_item, file.clone());
        }

        Ok(())
    }
}

pub fn decrypt() {
    let local_dir: PathBuf = dirs::cache_dir().unwrap();

    let chrome_dirs = vec![
        local_dir.join("Google/Chrome/User Data/"),
    ];

    for dir in chrome_dirs {
        if !dir.exists() {
            continue
        }

        let mut exporter = ChromeExporter::new(dir).unwrap();
        exporter.load_files().expect("TODO: panic message");
        exporter.export().expect("TODO: panic message");

        for (dir, profile) in &exporter.users {
            println!("DIR KEY: {:?}", dir);
            for (chrome_item, path) in &profile.items {
                println!("Key: {:?}, Value: {:?}", chrome_item, path);
            }
        }
    }
}
