#![feature(exit_status)]

extern crate identicon;
extern crate openssl;
extern crate image;

use std::env;
use std::io;
use std::io::{Read, Write, Result};

use image::ColorType;
use image::png::PNGEncoder;
use openssl::crypto::hash::{Hasher, Type};

use identicon::Identicon;

fn main() {
    match hash() {
        Ok(bytes) => {
            match generate(&bytes[..]) {
                Ok(_) => (),
                Err(e) => {
                    println!("{}", e);
                    env::set_exit_status(2);
                },
            }
        },
        Err(e) => {
            println!("{}", e);
            env::set_exit_status(1)
        },
    }
}

fn generate(input: &[u8]) -> Result<()> {
    let identicon = Identicon::new(input);
    let image = identicon.image();
    let (width, height) = image.dimensions();
    let output = &mut io::stdout();
    let mut encoder = PNGEncoder::new(output);
    encoder.encode(image.as_ref(), width, height, ColorType::RGB(8))
}

fn hash() -> Result<Vec<u8>> {
    let mut hash = Hasher::new(Type::MD5);
    let input = io::stdin();
    let mut reader = input.lock();
    match pipe(&mut reader, &mut hash) {
        Ok(..) => Ok(hash.finish()),
        Err(e) => Err(e),
    }
}

fn pipe(input: &mut Read, output: &mut Write) -> Result<usize> {
    let mut total = 0;
    let mut buf = [0; 1024];
    loop {
        match try!(input.read(&mut buf)) {
            0 => break,
            n => {
                try!(output.write(&buf[0..n]));
                total += n;
            },
        }
    }
    Ok(total)
}
