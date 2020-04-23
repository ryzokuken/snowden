extern crate gpgme;
extern crate xdg;

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("snowden").unwrap();
    let config_path = xdg_dirs.find_config_file("config.toml");
    if config_path.is_none() {
        println!("config.toml not found");
        std::process::exit(1);
    }
    println!("{:?}", config_path);
}
