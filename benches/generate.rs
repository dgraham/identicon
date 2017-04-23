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

    x.iter(|| match hash2(md5, input) {
        Ok(bytes) => Identicon::new(&bytes).image(),
        Err(e) => panic!(e),
    });
}
