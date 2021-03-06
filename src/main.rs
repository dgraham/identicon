use std::io;
use std::io::Result;
use std::process::exit;

use image::png::PNGEncoder;
use image::ColorType;
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
    encoder
        .encode(image.as_ref(), width, height, ColorType::Rgb8)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

fn hash() -> Result<[u8; 16]> {
    let input = io::stdin();
    let mut reader = input.lock();
    let mut digest = Md5::new();
    io::copy(&mut reader, &mut digest)?;

    let result = digest.result();

    let mut bytes = [0; 16];
    bytes.copy_from_slice(&result);
    Ok(bytes)
}
