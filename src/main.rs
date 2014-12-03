extern crate identicon;
extern crate openssl;
extern crate png;

use std::io;
use std::os;

use openssl::crypto::hash::{Hasher, HashType};
use png::store_png;

use identicon::Identicon;

fn main() {
    match output() {
        Some(path) => {
            match hash() {
                Some(bytes) => generate(bytes, &path),
                None => os::set_exit_status(2),
            }
        },
        None => {
            println!("Usage: identicon FILENAME INPUT");
            os::set_exit_status(1);
        }
    }
}

fn output() -> Option<Path> {
    if os::args().len() == 2 {
        let name = os::args()[1].clone();
        Some(Path::new(name))
    } else {
        None
    }
}

fn generate(input: Vec<u8>, path: &Path) {
    let identicon = Identicon::new(input);
    let mut image = identicon.image();
    let result = store_png(&mut image, path);
    match result {
        Ok(_) => {},
        Err(e) => {
            println!("{}", e);
            os::set_exit_status(2);
        },
    };
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
