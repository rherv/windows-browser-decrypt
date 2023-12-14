#[cfg(test)]
mod chrome {
    use windows_browser_decrypt::chrome::*;

    #[test]
    fn test_chrome() {
        let chrome_instances = export().unwrap();

        for instance in chrome_instances {
            for user in instance.get_users() {
                println!("Logins: {:?}", user.get_logins());
                println!("Cookies: {:?}", user.get_cookies());
                println!("History: {:?}", user.get_history());
                println!("Credit Cards: {:?}", user.get_credit_cards());
                println!("Items: {:?}", user.get_items())
            }
        }
    }
}