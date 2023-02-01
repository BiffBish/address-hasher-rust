use std::iter;
use std::time::Instant;

use crate::hmac;
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
pub fn pbkdf2(password: Vec<u8>, salt: Vec<u8>, count: u32, length: usize) -> Vec<u8> {
    let mut prf = hmac::Hmac::new(password);
    let mut u: Vec<u8>;
    let mut ui: Vec<u8>;
    let mut k: u32 = 1;
    let mut out: Vec<u8> = vec![];

    while 32 * out.len() < length {
        let thing: Vec<u8> = vec![
            salt.clone(),
            vec![(k >> 24) as u8, (k >> 16) as u8, (k >> 8) as u8, k as u8],
        ]
        .into_iter()
        .flatten()
        .collect();
        // res =

        ui = prf._encrypt(&thing);
        u = ui.clone();

        for i in 1..count {
            println!("i: {}", i);
            time_it!("  _encrypt loop", ui = prf._encrypt(&ui));
            // ui = prf._encrypt(&ui);
            for j in 0..ui.len() {
                u[j] ^= ui[j];
            }
        }

        out.append(&mut u);

        k += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn sha512Stage1() {
        let now = Instant::now();

        // for i in (0..100) {
        time_it!("pbkdf2" ,let res = pbkdf2(
            vec![
                0x73, 0x75, 0x72, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x20, 0x6d, 0x69, 0x73, 0x73, 0x20,
                0x6e, 0x6f, 0x6d, 0x69, 0x6e, 0x65, 0x65, 0x20, 0x64, 0x72, 0x65, 0x61, 0x6d, 0x20,
                0x67, 0x61, 0x70, 0x20, 0x63, 0x72, 0x6f, 0x73, 0x73, 0x20, 0x61, 0x73, 0x73, 0x61,
                0x75, 0x6c, 0x74, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x6b, 0x20, 0x63, 0x61, 0x70, 0x74,
                0x61, 0x69, 0x6e, 0x20, 0x70, 0x72, 0x6f, 0x73, 0x70, 0x65, 0x72, 0x20, 0x64, 0x72,
                0x6f, 0x70, 0x20, 0x64, 0x75, 0x74, 0x79, 0x20, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x20,
                0x63, 0x61, 0x6e, 0x64, 0x79, 0x20, 0x77, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x20, 0x77,
                0x65, 0x61, 0x74, 0x68, 0x65, 0x72, 0x20, 0x73, 0x63, 0x61, 0x6c, 0x65, 0x20, 0x70,
                0x75, 0x74,
            ],
            vec![0x6d, 0x6e, 0x65, 0x6d, 0x6f, 0x6e, 0x69, 0x63],
            2048,
            512,
        ));
        // }
        assert_eq!(
            res,
            vec![
                0x7e, 0xac, 0xf8, 0xe4, 0x21, 0xfd, 0x62, 0x6e, 0xa6, 0x65, 0x31, 0x75, 0x5f, 0xb7,
                0xeb, 0xe2, 0x3a, 0xd7, 0xd1, 0x54, 0xe8, 0x4a, 0xb1, 0x35, 0xf7, 0xa0, 0x8, 0xff,
                0x7c, 0xcf, 0x86, 0x7f, 0x4c, 0x78, 0x24, 0x3d, 0x2b, 0xa6, 0x45, 0x4e, 0xe3, 0x6a,
                0x89, 0xa1, 0x8, 0x72, 0xd4, 0x2d, 0x96, 0x39, 0x86, 0x3f, 0x88, 0x4a, 0xef, 0x69,
                0x25, 0xb8, 0x21, 0x7e, 0xc9, 0xf, 0x6f, 0x30
            ]
        );
        println!(
            "sha512Stage1: {}ms",
            now.elapsed().as_micros() as f64 / 1000.0
        );
    }
}