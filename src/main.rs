extern crate gpgme;
extern crate toml;
extern crate xdg;

fn get_key() -> gpgme::Key {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("snowden").unwrap();
    let config_path = xdg_dirs.find_config_file("config.toml");
    if config_path.is_none() {
        println!("config.toml not found");
        std::process::exit(1);
    }
    let config = std::fs::read_to_string(config_path.unwrap()).unwrap();
    let value = config.parse::<toml::Value>();
    if value.is_err() {
        println!("config.toml is invalid");
        std::process::exit(2);
    }
    let value = value.unwrap();
    let fpr = value["key"].as_str().unwrap();
    let mut ctx = gpgme::Context::from_protocol(gpgme::Protocol::OpenPgp).unwrap();
    let key = ctx.get_secret_key(fpr);
    if key.is_err() {
        println!("key with fingerprint {} is invalid", fpr);
        std::process::exit(3);
    }
    return key.unwrap();
}

fn main() {
    let key = get_key();
    println!("{:?}", key);
}
