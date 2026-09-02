#![cfg(feature = "build-bin")]

use std::io::Result;
use std::io::{self, Read};
use std::process::exit;

use image::ColorType;
use image::png::PngEncoder;
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
    let encoder = PngEncoder::new(output);
    encoder
        .encode(image.as_ref(), width, height, ColorType::Rgb8)
        .map_err(io::Error::other)
}

fn hash() -> Result<[u8; 16]> {
    let input = io::stdin();
    let mut reader = input.lock();
    let mut digest = Md5::new();
    let mut buffer = [0; 8 * 1024];

    while let bytes_read @ 1.. = reader.read(&mut buffer)? {
        digest.update(&buffer[..bytes_read]);
    }

    let result = digest.finalize();

    let mut bytes = [0; 16];
    bytes.copy_from_slice(&result);
    Ok(bytes)
}
