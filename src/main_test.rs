mod tests {
    use crate::*;
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
        let mut path = std::path::PathBuf::new();
        path.push("./target/test_dump/config.toml");
        std::fs::create_dir_all("./target/test_dump").unwrap();
        let mut file = std::fs::File::create(&path).expect("Error Creating config.toml");
        file.write_all(b"Hello, world!").expect("Error Writing config.toml");
        get_key_fingerprint(&path);
        std::fs::remove_file(&path).expect("Error Deleting config.toml")
    }

    #[test]
    fn valid_key_config() {
        let mut path = std::path::PathBuf::new();
        path.push("./target/test_dump/config.toml");
        std::fs::create_dir_all("./target/test_dump").unwrap();
        let mut file = std::fs::File::create(&path).expect("Error Creating config.toml");
        file.write_all(b"key='5744 6EFD E098 E5C9 34B6  9C7D C208 ADDE 26C2 B797'").expect("Error Writing config.toml");
        assert_eq!("5744 6EFD E098 E5C9 34B6  9C7D C208 ADDE 26C2 B797", get_key_fingerprint(&path));
        std::fs::remove_file(path).expect("Error Deleting config.toml")
    }

    #[test]
    #[should_panic]
    fn absence_of_xdg_config() {
        get_xdg_config();
    }

    #[test]
    fn presence_of_xdg_config() {
        let mut path = std::path::PathBuf::new();
        path.push("~/.config/snowden");
        std::fs::create_dir_all(&path).unwrap();
        std::fs::File::create("~/.config/snowden/config.toml").expect("Error Creating config.toml");
        assert_eq!(path,get_xdg_config());
    }
}
