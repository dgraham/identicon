#![feature(test)]
extern crate test;

use md5::{Digest, Md5};
use test::Bencher;
use identicon::Identicon;

#[bench]
fn generate(x: &mut Bencher) {
    let input = "42".as_bytes();
    let bytes = Md5::digest(input);
    let identicon = Identicon::new(&bytes);
    x.iter(|| identicon.image());
}
