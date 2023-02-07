// #![allow(dead_code)]
// #![allow(unused)]
use std::iter;
// mod hash;
mod bigInt;
mod curve;
mod ecPair;
mod hdNode;
mod hmac;
mod pbkdf2;
mod point;
mod sha512hash;

use hdNode::HDNode;
use profile::profile;
use sha256hash::Sha256Hash;

use crate::bech32::{encode, toWords};
mod bech32;
mod rmd160hash;
mod sha256hash;
use tokio;

pub static Secp256k1: once_cell::sync::Lazy<curve::Curve> =
    once_cell::sync::Lazy::new(|| curve::Curve::new());

static is_profiling: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[profile()]
fn utf8_string_to_bytes(s: &str) -> Vec<u8> {
    let mut result = Vec::new();
    for c in s.chars() {
        let mut buf = [0; 4];
        let s = c.encode_utf8(&mut buf);
        for b in s.bytes() {
            result.push(b);
        }
    }
    result
}

#[profile()]
fn to_seed(mnemonic: &str) -> Vec<u8> {
    let mnemonic_bits = utf8_string_to_bytes(mnemonic);
    let passphrase_bits = utf8_string_to_bytes("mnemonic");
    pbkdf2::pbkdf2(mnemonic_bits, passphrase_bits, 2048, 512)
}
#[profile()]
fn calcBip32ExtendedKey(bip32RootKey: HDNode) -> HDNode {
    let mut extendedKey = bip32RootKey;
    // Derive the key from the path
    let pathBits = "m/0/0".split("/");

    for i in [0, 0] {
        extendedKey = extendedKey.derive(i);
    }

    return extendedKey;
}
#[profile()]
fn cosmos_buffer_to_address(pubBuf: Vec<u8>) -> String {
    let mut sha = Sha256Hash::new();
    sha.update(&pubBuf);
    let temp = sha.digest();

    let mut ripemd160 = rmd160hash::Rrd160Hash::new();
    ripemd160.update(&temp);
    let ripemd160 = ripemd160.digest();

    encode("cosmos", &toWords(&ripemd160))
}

#[profile()]
fn from_mnemonic_to_address(mnemonic: &str, count: u64) -> String {
    let seed = to_seed(mnemonic);
    let hd = hdNode::HDNode::from_seed_buffer(&seed, "ed25519");
    let extendedKey = calcBip32ExtendedKey(hd);
    let pubKey = extendedKey.keyPair.getPublicKeyBuffer();
    cosmos_buffer_to_address(pubKey)
}

fn main() {
    let time = std::time::SystemTime::now();

    let address = from_mnemonic_to_address("surround miss nominee dream gap cross assault thank captain prosper drop duty group candy wealth weather scale put", 5);
    println!("{}", address);
    if address != "cosmos19x6j6a99rpfjkgchakhclqpghxavq8c2dgdqvw" {
        panic!("Wrong address");
    }
    let duration = time.elapsed().unwrap();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_seed_test() {
        let mnemonic = "surround miss nominee dream gap cross assault thank captain prosper drop duty group candy wealth weather scale put";
        let result = to_seed(mnemonic);
        assert_eq!(result, vec![]);
    }
}
