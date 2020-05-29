mod tests {
    use crate::*;

    #[test]
    #[should_panic]
    fn empty_config_path() {
        let path = std::path::PathBuf::new();
        get_key_fingerprint(path);
    }
}
