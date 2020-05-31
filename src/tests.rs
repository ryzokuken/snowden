mod get_key_fingerprint {
    use crate::get_key_fingerprint;
    use std::io::Write;

    #[test]
    #[should_panic]
    fn empty_config_path() {
        let path = std::path::PathBuf::new();
        get_key_fingerprint(&path);
    }

    #[test]
    #[should_panic]
    fn invalid_toml_config() {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(b"Hello, world!").unwrap();
        get_key_fingerprint(file.path());
        file.close().unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_key_config() {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(b"key=42").unwrap();
        get_key_fingerprint(file.path());
        file.close().unwrap();
    }

    #[test]
    fn valid_key_config() {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(b"key='test key fingerprint'").unwrap();
        assert_eq!("test key fingerprint", get_key_fingerprint(file.path()));
        file.close().unwrap();
    }
}
