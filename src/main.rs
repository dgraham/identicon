extern crate identicon;
extern crate image;
extern crate md5;

use std::io;
use std::io::Result;
use std::process::exit;

use image::ColorType;
use image::png::PNGEncoder;
use md5::{Digest, Md5};

use identicon::Identicon;

fn main() {
    match hash().and_then(|bytes| generate(&bytes)) {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }
}

fn generate(input: &[u8]) -> Result<()> {
    let identicon = Identicon::new(input);
    let image = identicon.image();
    let (width, height) = image.dimensions();
    let output = &mut io::stdout();
    let encoder = PNGEncoder::new(output);
    encoder.encode(image.as_ref(), width, height, ColorType::RGB(8))
}

fn hash() -> Result<[u8; 16]> {
    let input = io::stdin();
    let mut reader = input.lock();
    let digest = Md5::digest_reader(&mut reader)?;

    let mut bytes = [0; 16];
    bytes.copy_from_slice(&digest);
    Ok(bytes)
}
