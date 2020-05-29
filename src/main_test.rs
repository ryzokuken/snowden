//extern crate snowden;
use std::path::PathBuf;
use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn empty_config_path() {
        let path = PathBuf::new();
        get_key_fingerprint(path);
    }
}
