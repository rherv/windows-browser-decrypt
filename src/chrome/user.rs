use std::collections::HashMap;
use std::{fs, io};
use std::path::PathBuf;
use rusqlite::Connection;
use tempfile::TempDir;
use crate::chrome::item::{ChromeCookie, ChromeItem, ChromeLogin};
use crate::decrypt::aes_gcm_256;

#[derive(Clone)]
pub struct ChromeUser {
    pub tmp_dir: PathBuf,
    pub master_key: Vec<u8>,
    items: HashMap<PathBuf, ChromeItem>,
}
impl ChromeUser {
    pub fn new(master_key: Vec<u8>) -> io::Result<ChromeUser> {
        let cp = ChromeUser {
            tmp_dir: TempDir::new()?.into_path(),
            master_key,
            items: HashMap::new(),
        };

        Ok(cp)
    }

    pub fn add_item(&mut self, item: ChromeItem, file_path: PathBuf) {
        let dest = self.tmp_dir.join(ChromeItem::temp_name(&item));
        self.items.insert(dest.clone(), item.clone());

        fs::copy(file_path, dest).expect("TODO: panic message");
    }

    pub fn get_items(&self) -> Vec<ChromeItem> {
        self.items.clone().into_values().collect()
    }

    pub fn read_login_data(&self) -> Result<(), String> {
        let path = self.tmp_dir.join(ChromeItem::temp_name(&ChromeItem::LoginData));
        let conn = Connection::open(path.as_path()).map_err(|e| {
            e.to_string()
        })?;

        let mut stmt = match conn.prepare(
            "SELECT origin_url, username_value, password_value, date_created FROM logins"
        ) {
            Ok(s) => s,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        let login_iter = stmt.query_map([], |row| {
            let pwd_buf: Vec<u8> = row.get(2)?;
            let decoded_pwd = aes_gcm_256(self.master_key.clone().as_mut_slice(), pwd_buf).unwrap();

            Ok(ChromeLogin {
                origin_url: row.get(0)?,
                username_value: row.get(1)?,
                password_value: decoded_pwd,
                date_created: row.get(3)?,
            })
        }).map_err(|e| e.to_string())?;

        for login in login_iter {
            println!("Found login {:?}", login.unwrap())
        };

        Ok(())
    }

    pub fn get_cookies(&self) -> Result<Vec<ChromeCookie>, String> {
        let mut output = Vec::new();

        let path = self.tmp_dir.join(ChromeItem::temp_name(&ChromeItem::Cookies));
        let conn = Connection::open(path.as_path()).map_err(|e| {
            e.to_string()
        })?;

        let mut stmt = match conn.prepare(
            "SELECT name, encrypted_value, host_key, path, creation_utc, expires_utc, is_secure, is_httponly, has_expires, is_persistent FROM cookies"
        ) {
            Ok(s) => s,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        let login_iter = stmt.query_map([], |row| {
            let cookie_buf: Vec<u8> = row.get(1)?;
            let decoded_cookie = aes_gcm_256(self.master_key.clone().as_mut_slice(), cookie_buf).unwrap();

            Ok(ChromeCookie {
                name: row.get(0)?,
                value: decoded_cookie,
                host_key: row.get(2)?,
                path: row.get(3)?,
                creation_utc: row.get(4)?,
                expires_utc: row.get(5)?,
                is_secure: row.get(6)?,
                is_httponly: row.get(7)?,
                has_expires: row.get(8)?,
                is_persistent: row.get(9)?,
            })
        }).map_err(|e| e.to_string())?;

        for cookie in login_iter {
            match cookie {
                Ok(c) => output.push(c),
                Err(_) => {
                    continue
                }
            }
        };

        Ok(output)
    }

    pub fn read_web_data() -> Result<(), String>{

        Ok(())
    }
}
