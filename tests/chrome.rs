#[cfg(test)]
mod chrome {
    use windows_browser_decrypt::chrome_exporter::decrypt;

    #[test]
    fn test_chrome() {
        decrypt();
    }
}