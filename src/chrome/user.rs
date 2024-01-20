use std::collections::HashMap;
use std::{fs, io};
use std::path::{Path, PathBuf};
use rusqlite::Connection;
use tempfile::TempDir;
use crate::chrome::item::{ChromeCookie, ChromeCreditCard, ChromeHistory, ChromeItem, ChromeLogin};
use crate::decrypt::{aes_gcm_256, decrypt_value, dpapi_crypt_unprotect_data};
use crate::error::ExporterError;

#[derive(Clone)]
pub struct ChromeUser {
    pub tmp_dir: PathBuf,
    pub master_key: Option<Vec<u8>>,
    items: HashMap<PathBuf, ChromeItem>,
}
impl ChromeUser {
    pub fn new(master_key: Option<Vec<u8>>) -> io::Result<ChromeUser> {
        let cp = ChromeUser {
            tmp_dir: TempDir::new()?.into_path(),
            master_key,
            items: HashMap::new(),
        };

        Ok(cp)
    }

    pub fn add_item(&mut self, item: ChromeItem, file_path: impl AsRef<Path>) {
        let dest = self.tmp_dir.join(ChromeItem::temp_name(&item));
        self.items.insert(dest.clone(), item.clone());

        // TODO: this fails while copying cookies
        match fs::copy(file_path.as_ref().clone(), dest) {
            Ok(_) => {}
            Err(_) => {}
        }
    }

    pub fn get_items(&self) -> Vec<ChromeItem> {
        self.items.clone().into_values().collect()
    }

    pub fn get_logins(&self) -> Result<Vec<ChromeLogin>, ExporterError> {
        let path = self.tmp_dir.join(ChromeItem::temp_name(&ChromeItem::LoginData));
        let conn = Connection::open(path.as_path()).map_err(|e| {
            ExporterError::IO(e.to_string())
        })?;

        let mut stmt = match conn.prepare(
            "SELECT origin_url, username_value, password_value, date_created FROM logins"
        ) {
            Ok(s) => s,
            Err(e) => {
                return Err(ExporterError::IO(e.to_string()))
            }
        };

        let chrome_logins = stmt
            .query_map([], |row| {
                Ok(ChromeLogin {
                    origin_url: row.get(0)?,
                    username: row.get(1)?,
                    password: decrypt_value(self.master_key.clone(), row.get(2)?),
                    date_created: row.get(3)?,
                })
            }).unwrap()
            .filter_map(|cl| {
                let ok = cl.ok();
                ok
            })
            .collect();

        Ok(chrome_logins)
    }

    pub fn get_cookies(&self) -> Result<Vec<ChromeCookie>, ExporterError> {
        let path = self.tmp_dir.join(ChromeItem::temp_name(&ChromeItem::Cookies));
        let conn = Connection::open(path.as_path()).map_err(|e| {
            ExporterError::IO(e.to_string())
        })?;

        let mut stmt = match conn.prepare(
            "SELECT name, encrypted_value, host_key, path, creation_utc, expires_utc, is_secure, is_httponly, has_expires, is_persistent FROM cookies"
        ) {
            Ok(s) => s,
            Err(e) => {
                return Err(ExporterError::IO(e.to_string()))
            }
        };

        let chrome_cookies = stmt
            .query_map([], |row| {
                Ok(ChromeCookie{
                    name: row.get(0)?,
                    value: decrypt_value(self.master_key.clone(),row.get(1)?),
                    host_key: row.get(2)?,
                    path: row.get(3)?,
                    creation_utc: row.get(4)?,
                    expires_utc: row.get(5)?,
                    is_secure: row.get(6)?,
                    is_httponly: row.get(7)?,
                    has_expires: row.get(8)?,
                    is_persistent: row.get(9)?,
                })
            }).unwrap()
            .filter_map(|cc| cc.ok())
            .collect();

        Ok(chrome_cookies)
    }

    pub fn get_history(&self) -> Result<Vec<ChromeHistory>, ExporterError> {
        let path = self.tmp_dir.join(ChromeItem::temp_name(&ChromeItem::History));
        let conn = Connection::open(path.as_path()).map_err(|e| {
            ExporterError::IO(e.to_string())
        })?;

        let mut stmt = match conn.prepare(
            "SELECT url, title, visit_count, last_visit_time FROM urls"
        ) {
            Ok(s) => s,
            Err(e) => {
                return Err(ExporterError::IO(e.to_string()))
            }
        };

        let chrome_history = stmt
            .query_map([], |row| {
                Ok(ChromeHistory{
                    url: row.get(0)?,
                    title: row.get(1)?,
                    visit_count: row.get(2)?,
                    last_visit_time: row.get(3)?,
                })
            }).unwrap()
            .filter_map(|cc| cc.ok())
            .collect();

        Ok(chrome_history)
    }

    pub fn get_credit_cards(&self) -> Result<Vec<ChromeCreditCard>, ExporterError> {
        let path = self.tmp_dir.join(ChromeItem::temp_name(&ChromeItem::WebData));
        let conn = Connection::open(path.as_path()).map_err(|e| {
            ExporterError::IO(e.to_string())
        })?;

        let mut stmt = match conn.prepare(
            "SELECT guid, name_on_card, expiration_month, expiration_year, card_number_encrypted, billing_address_id, nickname FROM credit_cards"
        ) {
            Ok(s) => s,
            Err(e) => {
                return Err(ExporterError::IO(e.to_string()))
            }
        };

        let chrome_credit_card = stmt
            .query_map([], |row| {
                Ok(ChromeCreditCard{
                    guid: row.get(0)?,
                    name_on_card: row.get(1)?,
                    expiration_month: row.get(2)?,
                    expiration_year: row.get(3)?,
                    card_number: decrypt_value(self.master_key.clone(), row.get(4)?),
                    billing_address_id: row.get(5)?,
                    nickname: row.get(6)?,
                })
            }).unwrap()
            .filter_map(|cc| cc.ok())
            .collect();

        Ok(chrome_credit_card)
    }
}
