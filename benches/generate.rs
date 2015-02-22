extern crate identicon;
extern crate openssl;
extern crate test;

use openssl::crypto::hash::{hash, Type};
use test::Bencher;

use identicon::Identicon;

#[bench]
fn generate(x: &mut Bencher) {
    let source = String::from_str("42");
    let input = source.as_bytes();

    x.iter(|| {
        let bytes = hash(Type::MD5, input);
        Identicon::new(&bytes[0..]).image();
    });
}
