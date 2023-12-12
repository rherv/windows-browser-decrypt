use std::path::Path;

#[derive(Debug)]
pub struct ChromeLogin {
    pub origin_url: String,
    pub username_value: String,
    pub password_value: String,
    pub date_created: i64,
}

#[derive(Debug)]
pub struct ChromeCookie {
    pub name: String,
    pub value: String,
    pub host_key: String,
    pub path: String,
    pub creation_utc: u64,
    pub expires_utc: u64,
    pub is_secure: bool,
    pub is_httponly: bool,
    pub has_expires: bool,
    pub is_persistent: bool,
}

#[derive(Debug)]
pub struct ChromeHistory {
    pub url: String,
    pub title: String,
    pub visit_count: u64,
    pub last_visit_time: u64,
}

#[derive(Debug)]
pub struct ChromeDownload {
    pub target_path: String,
    pub tab_url: String,
    pub total_bytes: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub mime_type: String,
}

#[derive(Debug)]
pub struct ChromeBookmark {
}

#[derive(Debug)]
pub struct ChromeCreditCard {
    pub guid: String,
    pub name_on_card: String,
    pub expiration_month: u64,
    pub expiration_year: u64,
    pub card_number: String,
    pub billing_address_id: String,
    pub nickname: String,
}

#[derive(Clone, Debug)]  // Derive the Clone trait for ChromeItem
#[derive(Eq, Hash, PartialEq)]
pub enum ChromeItem {
    LoginData,
    WebData,
    History,
    Cookies,
    Bookmarks,
    LocalStorage,
    SessionStorage,
    Extensions,
}


impl ChromeItem {
    pub fn new(file_path: impl AsRef<Path>) -> Result<ChromeItem, String> {
        let name = match file_path.as_ref().file_name() {
            Some(n) => n.to_os_string(),
            None => {
                return Err("Failed to get filename.".to_string());
            }
        };

        Ok(match name.to_os_string().to_string_lossy().into_owned().as_str() {
            "Login Data" => ChromeItem::LoginData,
            "Web Data" => ChromeItem::WebData,
            "History" => ChromeItem::History,
            "Cookies" => ChromeItem::Cookies,
            "Bookmarks" => ChromeItem::Bookmarks,
            "leveldb" => ChromeItem::LocalStorage,
            "Session Storage" => ChromeItem::SessionStorage,
            "Extensions" => ChromeItem::Extensions,
            _ => {
                return Err("File is not a exporter item.".to_string());
            }
        })

    }

    pub fn temp_name(item: &ChromeItem) -> String {
        match item {
            ChromeItem::LoginData => "logindata",
            ChromeItem::WebData => "webdata",
            ChromeItem::History => "history",
            ChromeItem::Cookies => "cookies",
            ChromeItem::Bookmarks => "bookmarks",
            ChromeItem::LocalStorage => "localstorage",
            ChromeItem::SessionStorage => "sessionstorage",
            ChromeItem::Extensions => "extensions",
        }.to_string()
    }
}
