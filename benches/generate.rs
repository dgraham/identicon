extern crate identicon;
extern crate openssl;
extern crate test;

use openssl::crypto::hash::{Hasher, HashType};
use test::Bencher;

use identicon::Identicon;

#[bench]
fn generate(x: &mut Bencher) {
    let source = String::from_str("42");
    let input = source.as_bytes();

    x.iter(|| {
        let mut hash = Hasher::new(HashType::MD5);
        hash.update(input);
        Identicon::new(hash.finalize()).image();
    });
}
