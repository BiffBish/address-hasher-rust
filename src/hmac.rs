use std::{iter, time::Instant};

use crate::sha512hash;
macro_rules! time_it {
    ($context:literal, $($s:stmt);+) => {
        let timer = std::time::Instant::now();
        $(
            $s
        )*
        println!("{}: {:?}", $context, timer.elapsed());
    }
}
pub struct Hmac {
    key: Vec<u8>,
    ipad: Vec<u8>,
    opad: Vec<u8>,
    hash: sha512hash::Sha512Hash,
}

fn printHex(prefix: &str, itt: impl Iterator<Item = u8>) {
    print!("{}  ", prefix);

    // prefix: ff, ff, ff, ff, ff, ff, ff, ff, ff
    //         ff, ff, ff, ff, ff, ff, ff, ff, ff
    let mut count = 0;
    for i in itt {
        if count % 16 == 0 && count != 0 {
            let padding = iter::repeat(" ").take(prefix.len()).collect::<String>();
            println!();
            print!("{}  ", padding);
        }

        print!("{:02x}, ", i);
        count += 1;
    }
    println!();
    println!();
}

impl Hmac {
    pub fn new(key: Vec<u8>) -> Hmac {
        let blocksize = 128;
        let mut ipad = vec![0; blocksize];
        let mut opad = vec![0; blocksize];
        let mut hash = sha512hash::Sha512Hash::new();
        let mut newKey: &[u8];
        if key.len() > blocksize {
            hash.update(&key);
            newKey = &hash.digest();
        } else {
            // Convert vec to [u8; 64]
            newKey = &key;
        }
        for i in 0..key.len() {
            ipad[i] = 54 ^ key[i];
            opad[i] = 92 ^ key[i];
        }
        for i in key.len()..blocksize {
            ipad[i] = 54;
            opad[i] = 92;
        }

        hash.update(&ipad);
        Hmac {
            key: key,
            ipad: ipad,
            opad: opad,
            hash: hash,
        }
    }

    pub fn _final(&mut self) -> Vec<u8> {
        // time_it!("_final",
        //     let h = self.hash.digest();
        //     self.hash.reset();
        //     self.hash.update(&self.opad);
        //     self.hash.update(&h);
        //     let res = self.hash.digest();
        //     self.hash.reset()
        // );

        // time_it!("  _final digest",   let h = self.hash.digest());
        // time_it!("  _final reset", self.hash.reset());
        // time_it!("  _final update", self.hash.update(&self.opad));
        // time_it!("  _final update2", self.hash.update(&h));
        // time_it!("  _final digest2",   let res = self.hash.digest());
        // time_it!("  _final reset", self.hash.reset());

        let h = self.hash.digest();
        self.hash.reset();
        self.hash.update(&self.opad);
        self.hash.update(&h);
        let res = self.hash.digest();
        self._reset();
        res
    }

    pub fn _encrypt(&mut self, data: &Vec<u8>) -> Vec<u8> {
        // printHex("[HMAC] ipad", self.ipad.iter().cloned());
        // time_it!("_encrypt update", self.hash.update(&data));
        // time_it!("_encrypt _final", let a = self._final());
        // a
        // printHex("[HMAC] data", data.iter().cloned());
        self.hash.update(&data);
        self._final()
    }

    pub fn _reset(&mut self) {
        self.hash.reset();
        self.hash.update(&self.ipad);
    }
}
#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn sha512Stage1() {
        let now = Instant::now();

        // for i in (0..100000) {
        // time_it("hmac",
        let mut hmac = Hmac::new(vec![0x6b, 0x65, 0x79]);
        let res = hmac._encrypt(&vec![
            0x54, 0x68, 0x65, 0x20, 0x71, 0x75, 0x69, 0x63, 0x6b, 0x20, 0x62, 0x72, 0x6f, 0x77,
            0x6e, 0x20, 0x66, 0x6f, 0x78, 0x20, 0x6a, 0x75, 0x6d, 0x70, 0x73, 0x20, 0x6f, 0x76,
            0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x7a, 0x79, 0x20, 0x64, 0x6f,
            0x67,
        ]);
        // );
        // }
        // assert_eq!(
        //         res,
        //         vec![
        //             0xb4, 0x2a, 0xf0, 0x90, 0x57, 0xba, 0xc1, 0xe2, 0xd4, 0x17, 0x08, 0xe4, 0x8a,
        //             0x90, 0x2e, 0x09, 0xb5, 0xff, 0x7f, 0x12, 0xab, 0x42, 0x8a, 0x4f, 0xe8, 0x66,
        //             0x53, 0xc7, 0x3d, 0xd2, 0x48, 0xfb, 0x82, 0xf9, 0x48, 0xa5, 0x49, 0xf7, 0xb7,
        //             0x91, 0xa5, 0xb4, 0x19, 0x15, 0xee, 0x4d, 0x1e, 0xc3, 0x93, 0x53, 0x57, 0xe4,
        //             0xe2, 0x31, 0x72, 0x50, 0xd0, 0x37, 0x2a, 0xfa, 0x2e, 0xbe, 0xeb, 0x3a,
        //         ]
        //     )
        println!(
            "Time elapsed in expensive_function() is: {:?}",
            now.elapsed()
        );
    }
}
