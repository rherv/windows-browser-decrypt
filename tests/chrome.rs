#[cfg(test)]
mod chrome {
    use windows_browser_decrypt::chrome::*;

    #[test]
    fn test_chrome() {
        let chrome_instances = export().unwrap();

        for instance in chrome_instances {
            for user in instance.get_users() {
                println!("{:?}", user.get_cookies().unwrap_or(Vec::new()));
                println!("{:?}", user.get_logins().unwrap());
                println!("{:?}", user.get_history().unwrap());
                println!("{:?}", user.get_credit_cards().unwrap());
                println!("{:?}", user.get_items())
            }
        }
    }
}