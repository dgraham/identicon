#![feature(test)]

extern crate identicon;
extern crate openssl;
extern crate test;

use openssl::hash::{hash2, MessageDigest};
use test::Bencher;

use identicon::Identicon;

#[bench]
fn generate(x: &mut Bencher) {
    let input = "42".as_bytes();
    let md5 = MessageDigest::md5();
    let bytes = hash2(md5, input).unwrap();
    let identicon = Identicon::new(&bytes);
    x.iter(|| identicon.image());
}
