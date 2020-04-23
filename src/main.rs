extern crate gpgme;

fn main() {
    let mut ctx = gpgme::Context::from_protocol(gpgme::Protocol::OpenPgp).unwrap();
    for key in ctx.secret_keys().unwrap() {
        println!("{:?}", key);
    }
}
