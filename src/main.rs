extern crate identicon;
extern crate image;
extern crate openssl;

use std::io;
use std::io::{Read, Write, Result};
use std::process::exit;

use image::ColorType;
use image::png::PNGEncoder;
use openssl::hash::{DigestBytes, Hasher, MessageDigest};

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

fn hash() -> Result<DigestBytes> {
    let mut hash = Hasher::new(MessageDigest::md5())?;
    let input = io::stdin();
    let mut reader = input.lock();
    pipe(&mut reader, &mut hash)?;
    Ok(hash.finish2()?)
}

fn pipe(input: &mut Read, output: &mut Write) -> Result<usize> {
    let mut total = 0;
    let mut buf = [0; 1024];
    loop {
        match input.read(&mut buf)? {
            0 => break,
            n => {
                output.write(&buf[0..n])?;
                total += n;
            }
        }
    }
    Ok(total)
}
