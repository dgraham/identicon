extern crate identicon;
extern crate openssl;
extern crate png;

use std::io;
use std::os;

use openssl::crypto::hash::{Hasher, HashType};
use png::write_png;

use identicon::Identicon;

fn main() {
    match hash() {
        Some(bytes) => {
            match generate(bytes) {
                Ok(_) => (),
                Err(e) => {
                    println!("{}", e);
                    os::set_exit_status(2);
                },
            }
        },
        None => os::set_exit_status(1),
    }
}

fn generate(input: Vec<u8>) -> Result<(), String> {
    let identicon = Identicon::new(input);
    let mut image = identicon.image();
    write_png(&mut image, &mut io::stdout())
}

fn hash() -> Option<Vec<u8>> {
    let mut hash = Hasher::new(HashType::MD5);
    for result in io::stdin().lines() {
        match result {
            Ok(line) => {
                hash.update(line.as_bytes());
            },
            Err(e) => {
                println!("{}", e);
                return None
            },
        }
    }
    Some(hash.finalize())
}
