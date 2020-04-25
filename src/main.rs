extern crate clap;
extern crate git2;
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
    key.unwrap()
}

fn get_repo() -> git2::Repository {
    let path = std::env::current_dir();
    if path.is_err() {
        println!("current directory is invalid");
        std::process::exit(4);
    }
    let path = path.unwrap();
    let repo = git2::Repository::open(path);
    if repo.is_err() {
        println!("current directory is not a valid git repository");
        std::process::exit(5);
    }
    repo.unwrap()
}

fn commit(key: gpgme::Key, repo: git2::Repository, msg: &str) {
    let head = repo.head();
    if head.is_err() {
        println!("error fetching head");
        std::process::exit(6);
    }
    let head = head.unwrap();
    let target = head.target().unwrap();
    let object = repo.find_object(target, None).unwrap();
    let prev_commit = object.as_commit();
    if prev_commit.is_none() {
        println!("head doesn't point to a commit");
        std::process::exit(7);
    }
    let prev_commit = prev_commit.unwrap();
    let signature = git2::Signature::now(key.id().unwrap(), "anon@ymo.us").unwrap();
    let commit = repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        msg,
        &prev_commit.tree().unwrap(),
        &[prev_commit],
    );
    if commit.is_err() {
        println!("failed to create commit");
        std::process::exit(8);
    }
}

fn main() {
    let key = get_key();
    let repo = get_repo();
    let matches = clap::App::new("snowden")
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
    if let Some(matches) = matches.subcommand_matches("commit") {
        commit(key, repo, matches.value_of("MESSAGE").unwrap());
    }
}
