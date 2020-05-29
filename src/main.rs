extern crate clap;
extern crate git2;
extern crate gpgme;
extern crate toml;
extern crate xdg;

#[cfg(test)]
mod main_test;

fn get_key_fingerprint(config_path: std::path::PathBuf) -> String {
    let config = std::fs::read_to_string(config_path)
        .expect("error reading config.toml");
    let value = config
        .parse::<toml::Value>()
        .expect("config.toml is invalid");
    String::from(value["key"].as_str().expect("key fingerprint invalid"))
}

fn get_key(fpr: &str) -> gpgme::Key {
    let mut ctx = gpgme::Context::from_protocol(gpgme::Protocol::OpenPgp)
        .expect("failed to connect to gpg");
    ctx.get_secret_key(fpr).expect("invalid key specified")
}

fn get_xdg_config() -> std::path::PathBuf {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("snowden")
        .expect("cannot find config directory");
    xdg_dirs
        .find_config_file("config.toml")
        .expect("config.toml not found")
}

fn get_repo() -> git2::Repository {
    let path = std::env::current_dir().expect("current directory is invalid");
    git2::Repository::open(path)
        .expect("current directory is not a valid git repository")
}

fn commit(key: gpgme::Key, repo: git2::Repository, msg: &str) {
    let head = repo.head().expect("error fetching head");
    let target = head.target().unwrap();
    let object = repo.find_object(target, None).unwrap();
    let prev_commit =
        object.as_commit().expect("head doesn't point to a commit");
    let signature =
        git2::Signature::now(key.id().unwrap(), "anon@ymo.us").unwrap();
    if repo
        .diff_tree_to_index(Some(&prev_commit.tree().unwrap()), None, None)
        .unwrap()
        .stats()
        .unwrap()
        .files_changed()
        == 0
    {
        panic!("no staged changed");
    }
    let new_target = repo.index().unwrap().write_tree().unwrap();
    let new_object = repo.find_object(new_target, None).unwrap();
    let new_tree = new_object.as_tree().unwrap();
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        msg,
        &new_tree,
        &[prev_commit],
    )
    .expect("failed to create commit");
}

fn main() {
    let repo = get_repo();
    let matches = clap::App::new("snowden")
        .arg(
            clap::Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .value_name("FILE"),
        )
        .arg(
            clap::Arg::with_name("key")
                .short("k")
                .long("key")
                .takes_value(true)
                .value_name("FINGERPRINT"),
        )
        .subcommand(
            clap::SubCommand::with_name("commit").arg(
                clap::Arg::with_name("MESSAGE")
                    .short("m")
                    .long("message")
                    .required(true)
                    .takes_value(true),
            ),
        )
        .get_matches();
    let fpr = match matches.value_of("key") {
        Some(fpr) => String::from(fpr),
        None => {
            let config_path = match matches.value_of("config") {
                Some(file) => std::path::PathBuf::from(file),
                None => get_xdg_config(),
            };
            get_key_fingerprint(config_path)
        }
    };
    let key = get_key(fpr.as_str());
    if let Some(matches) = matches.subcommand_matches("commit") {
        commit(key, repo, matches.value_of("MESSAGE").unwrap());
    }
}
