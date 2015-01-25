extern crate identicon;
extern crate openssl;
extern crate image;

use std::io;
use std::io::IoError;
use std::os;

use image::ColorType;
use image::png::PNGEncoder;
use openssl::crypto::hash::{Hasher, HashType};

use identicon::Identicon;

fn main() {
    match hash() {
        Some(bytes) => {
            match generate(&bytes[0..]) {
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

fn generate(input: &[u8]) -> Result<(), IoError> {
    let identicon = Identicon::new(input);
    let image = identicon.image();
    let (width, height) = image.dimensions();
    let mut encoder = PNGEncoder::new(io::stdout());
    encoder.encode(image.as_slice(), width, height, ColorType::RGB(8))
}

fn hash() -> Option<Vec<u8>> {
    let mut hash = Hasher::new(HashType::MD5);
    for result in io::stdin().lock().lines() {
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
