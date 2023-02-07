use std::thread::sleep;
use std::time::{Duration, Instant};

// #![allow(arithmetic_overflow)]

const zl: [usize; 80] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 7, 4, 13, 1, 10, 6, 15, 3, 12, 0, 9, 5,
    2, 14, 11, 8, 3, 10, 14, 4, 9, 15, 8, 1, 2, 7, 0, 6, 13, 11, 5, 12, 1, 9, 11, 10, 0, 8, 12, 4,
    13, 3, 7, 15, 14, 5, 6, 2, 4, 0, 5, 9, 7, 12, 2, 10, 14, 1, 3, 8, 11, 6, 15, 13,
];
const zr: [usize; 80] = [
    5, 14, 7, 0, 9, 2, 11, 4, 13, 6, 15, 8, 1, 10, 3, 12, 6, 11, 3, 7, 0, 13, 5, 10, 14, 15, 8, 12,
    4, 9, 1, 2, 15, 5, 1, 3, 7, 14, 6, 9, 11, 8, 12, 2, 10, 0, 4, 13, 8, 6, 4, 1, 3, 11, 15, 0, 5,
    12, 2, 13, 9, 7, 10, 14, 12, 15, 10, 4, 1, 5, 8, 7, 6, 2, 13, 14, 0, 3, 9, 11,
];
const sl: [u32; 80] = [
    11, 14, 15, 12, 5, 8, 7, 9, 11, 13, 14, 15, 6, 7, 9, 8, 7, 6, 8, 13, 11, 9, 7, 15, 7, 12, 15,
    9, 11, 7, 13, 12, 11, 13, 6, 7, 14, 9, 13, 15, 14, 8, 13, 6, 5, 12, 7, 5, 11, 12, 14, 15, 14,
    15, 9, 8, 9, 14, 5, 6, 8, 6, 5, 12, 9, 15, 5, 11, 6, 8, 13, 12, 5, 12, 13, 14, 11, 8, 5, 6,
];
const sr: [u32; 80] = [
    8, 9, 9, 11, 13, 15, 15, 5, 7, 7, 8, 11, 14, 14, 12, 6, 9, 13, 15, 7, 12, 8, 9, 11, 7, 7, 12,
    7, 6, 15, 13, 11, 9, 7, 15, 11, 8, 6, 6, 14, 12, 13, 5, 14, 13, 13, 7, 5, 15, 5, 8, 11, 14, 14,
    6, 14, 6, 9, 12, 9, 12, 5, 15, 8, 8, 5, 12, 9, 12, 5, 14, 6, 8, 13, 6, 5, 15, 13, 11, 11,
];

const hl: [u32; 5] = [0, 1518500249, 1859775393, 2400959708, 2840853838];
const hr: [u32; 5] = [1352829926, 1548603684, 1836072691, 2053994217, 0];
// const ARRAY16 = new Array(16);
fn rotl(x: u32, n: u32) -> u32 {
    return (x << n) | (x >> (32 - n));
}
fn fn1(a: u32, b: u32, c: u32, d: u32, e: u32, m: u32, k: u32, s: u32) -> u32 {
    return (u32::wrapping_add(
        rotl(
            u32::wrapping_add(u32::wrapping_add(a, b ^ c ^ d), u32::wrapping_add(m, k)),
            s,
        ),
        e,
    ));
}
fn fn2(a: u32, b: u32, c: u32, d: u32, e: u32, m: u32, k: u32, s: u32) -> u32 {
    // return (rotl((a + ((b & c) | (!b & d)) + m + k) | 0, s) + e) | 0;
    return (u32::wrapping_add(
        rotl(
            u32::wrapping_add(
                u32::wrapping_add(a, (b & c) | (!b & d)),
                u32::wrapping_add(m, k),
            ),
            s,
        ),
        e,
    ));
}
fn fn3(a: u32, b: u32, c: u32, d: u32, e: u32, m: u32, k: u32, s: u32) -> u32 {
    // return (rotl((a + ((b | !c) ^ d) + m + k) | 0, s) + e) | 0;
    return (u32::wrapping_add(
        rotl(
            u32::wrapping_add(u32::wrapping_add(a, (b | !c) ^ d), u32::wrapping_add(m, k)),
            s,
        ),
        e,
    ));
}
fn fn4(a: u32, b: u32, c: u32, d: u32, e: u32, m: u32, k: u32, s: u32) -> u32 {
    // return (rotl((a + ((b & d) | (c & !d)) + m + k) | 0, s) + e) | 0;
    return (u32::wrapping_add(
        rotl(
            u32::wrapping_add(
                u32::wrapping_add(a, (b & d) | (c & !d)),
                u32::wrapping_add(m, k),
            ),
            s,
        ),
        e,
    ));
}
fn fn5(a: u32, b: u32, c: u32, d: u32, e: u32, m: u32, k: u32, s: u32) -> u32 {
    // return (rotl((a + (b ^ (c | !d)) + m + k) | 0, s) + e) | 0;
    return (u32::wrapping_add(
        rotl(
            u32::wrapping_add(u32::wrapping_add(a, b ^ (c | !d)), u32::wrapping_add(m, k)),
            s,
        ),
        e,
    ));
}

const BLOCK_SIZE: usize = 64;
const FINAL_SIZE: usize = 112;

pub struct Rrd160Hash {
    _block: [u8; BLOCK_SIZE],
    _len: u64,

    _block_offset: usize,

    _a: u32,
    _b: u32,
    _c: u32,
    _d: u32,
    _e: u32,
}

impl Rrd160Hash {
    pub fn new() -> Rrd160Hash {
        return Rrd160Hash {
            _block: [0; BLOCK_SIZE],
            _len: 0,
            _block_offset: 0,
            _a: 1732584193,
            _b: 4023233417,
            _c: 2562383102,
            _d: 271733878,
            _e: 3285377520,
        };
    }

    pub fn update(&mut self, data: &[u8]) -> &mut Rrd160Hash {
        // if (this._finalized) throw new Error("Digest already called");

        let mut offset = 0;

        while self._block_offset + data.len() - offset >= BLOCK_SIZE {
            for i in self._block_offset..BLOCK_SIZE {
                panic!("m");
                self._block[i] = data[offset];
                offset += 1;
                self._update();
                self._block_offset = 0;
            }
        }

        while offset < data.len() {
            self._block[self._block_offset] = data[offset];
            offset += 1;
            self._block_offset += 1;
        }

        let mut carry = 8 * data.len();
        let mut j = 0;
        while carry > 0 {
            self._len += carry as u64;
            carry = (self._len / 4294967296) as usize;
            if carry > 0 {
                self._len -= 4294967296 * carry as u64;
            }
        }

        return self;
    }

    pub fn reset(&mut self) {
        self._block.fill(0);
        self._len = 0;
        self._block_offset = 0;
        self._a = 1732584193;
        self._b = 4023233417;
        self._c = 2562383102;
        self._d = 271733878;
        self._e = 3285377520;
    }

    pub fn digest(&mut self) -> Vec<u8> {
        let digest = self._hash();

        self._block.fill(0);
        self._block_offset = 0;
        self._len = 0;
        return digest;
    }

    pub fn _update(&mut self) {
        let mut words = vec![0u32; 16];
        for i in 0..16 {
            words[i] = (self._block[i * 4] as u32)
                | ((self._block[i * 4 + 1] as u32) << 8)
                | ((self._block[i * 4 + 2] as u32) << 16)
                | ((self._block[i * 4 + 3] as u32) << 24);
        }

        let mut al = self._a as u32;
        let mut bl = self._b as u32;
        let mut cl = self._c as u32;
        let mut dl = self._d as u32;
        let mut el = self._e as u32;
        let mut ar = self._a as u32;
        let mut br = self._b as u32;
        let mut cr = self._c as u32;
        let mut dr = self._d as u32;
        let mut er = self._e as u32;

        for i in 0..80 {
            let mut tl = 0;
            let mut tr = 0;
            if i < 16 {
                tl = fn1(al, bl, cl, dl, el, words[zl[i]], hl[0], sl[i]);
                tr = fn5(ar, br, cr, dr, er, words[zr[i]], hr[0], sr[i]);
            } else if i < 32 {
                tl = fn2(al, bl, cl, dl, el, words[zl[i]], hl[1], sl[i]);
                tr = fn4(ar, br, cr, dr, er, words[zr[i]], hr[1], sr[i]);
            } else if i < 48 {
                tl = fn3(al, bl, cl, dl, el, words[zl[i]], hl[2], sl[i]);
                tr = fn3(ar, br, cr, dr, er, words[zr[i]], hr[2], sr[i]);
            } else if i < 64 {
                tl = fn4(al, bl, cl, dl, el, words[zl[i]], hl[3], sl[i]);
                tr = fn2(ar, br, cr, dr, er, words[zr[i]], hr[3], sr[i]);
            } else {
                tl = fn5(al, bl, cl, dl, el, words[zl[i]], hl[4], sl[i]);
                tr = fn1(ar, br, cr, dr, er, words[zr[i]], hr[4], sr[i]);
            }
            al = el;
            el = dl;
            dl = rotl(cl, 10);
            cl = bl;
            bl = tl;
            ar = er;
            er = dr;
            dr = rotl(cr, 10);
            cr = br;
            br = tr;
        }

        let t = u32::wrapping_add(u32::wrapping_add(self._b, cl), dr) as u32;
        // self._b = (self._c + dl + er) as u32;
        self._b = u32::wrapping_add(u32::wrapping_add(self._c, dl), er) as u32;

        self._c = u32::wrapping_add(u32::wrapping_add(self._d, el), ar) as u32;
        // self._c = (self._d + el + ar) as u32;
        self._d = u32::wrapping_add(u32::wrapping_add(self._e, al), br) as u32;
        // self._d = (self._e + al + br) as u32;
        self._e = u32::wrapping_add(u32::wrapping_add(self._a, bl), cr) as u32;
        // self._e = (self._a + bl + cr) as u32;
        self._a = t as u32;
    }
    pub fn _hash(&mut self) -> Vec<u8> {
        self._block[self._block_offset] = 128;
        self._block_offset += 1;
        // self._block_offset > 56 &&
        //   (self._block.fill(0, self._block_offset, 64),
        //   self._update(),
        //   (self._block_offset = 0)),
        //   self._block.fill(0, self._block_offset, 56),
        //   self._block.writeUInt32LE(self._length[0], 56),
        //   self._block.writeUInt32LE(self._length[1], 60),
        //   self._update();
        if self._block_offset > 56 {
            for i in 56..64 {
                self._block[i] = 0;
            }
            self._update();
            self._block_offset = 0;
        }
        // self._block.fill(0, self._block_offset, 56);
        for i in self._block_offset..56 {
            self._block[i] = 0;
        }
        self._block[56] = (self._len & 0xff) as u8;
        self._block[57] = ((self._len >> 8) & 0xff) as u8;
        self._update();

        let mut data = vec![0u8; 20];

        fn write_u32_le(data: &mut [u8], offset: usize, value: u32) {
            data[offset] = (value & 0xff) as u8;
            data[offset + 1] = ((value >> 8) & 0xff) as u8;
            data[offset + 2] = ((value >> 16) & 0xff) as u8;
            data[offset + 3] = ((value >> 24) & 0xff) as u8;
        }

        write_u32_le(&mut data, 0, self._a);
        write_u32_le(&mut data, 4, self._b);
        write_u32_le(&mut data, 8, self._c);
        write_u32_le(&mut data, 12, self._d);
        write_u32_le(&mut data, 16, self._e);
        return data;

        // return (
        //   buffer.writeInt32LE(self._a, 0),
        //   buffer.writeInt32LE(self._b, 4),
        //   buffer.writeInt32LE(self._c, 8),
        //   buffer.writeInt32LE(self._d, 12),
        //   buffer.writeInt32LE(self._e, 16),
        //   buffer
        // );
    }
}

pub fn main() {
    println!("Hello, world!");
}

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn rmd160() {
//         let now = Instant::now();

//         let mut sha = Sha512Hash::new();
//         sha._update([
//             0x54, 0x68, 0x65, 0x20, 0x71, 0x75, 0x69, 0x63, 0x6b, 0x20, 0x62, 0x72, 0x6f, 0x77,
//             0x6e, 0x20, 0x66, 0x6f, 0x78, 0x20, 0x6a, 0x75, 0x6d, 0x70, 0x73, 0x20, 0x6f, 0x76,
//             0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x7a, 0x79, 0x20, 0x64, 0x6f,
//             0x67, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//             0x00, 0x00,
//         ]);

//         let hash = sha._hash();

//         assert_eq!(
//             hash,
//             [
//                 0xb2, 0xdf, 0x56, 0xf0, 0x4c, 0x42, 0x8e, 0xf0, 0x51, 0xc0, 0xc4, 0x74, 0x67, 0x89,
//                 0xe6, 0xf6, 0xa8, 0xf9, 0x96, 0x87, 0xe9, 0xaa, 0x19, 0x1b, 0xef, 0x40, 0x42, 0x43,
//                 0x40, 0x21, 0x75, 0x4b, 0x21, 0xe1, 0x2c, 0xa6, 0x17, 0x99, 0x9e, 0x85, 0x07, 0x15,
//                 0x70, 0x44, 0x70, 0x84, 0x4f, 0xad, 0x79, 0x37, 0x26, 0xad, 0xc4, 0xf7, 0x47, 0xa3,
//                 0x81, 0x5d, 0xe0, 0xc7, 0x56, 0x55, 0x0a, 0xb3,
//             ]
//         );
//     }

//     #[test]
//     fn sha512_stage2() {
//         let now = Instant::now();

//         let mut sha = Sha512Hash::new();
//         sha.update(
//             vec![
//                 0x54, 0x68, 0x65, 0x20, 0x71, 0x75, 0x69, 0x63, 0x6b, 0x20, 0x62, 0x72, 0x6f, 0x77,
//                 0x6e, 0x20, 0x66, 0x6f, 0x78, 0x20, 0x6a, 0x75, 0x6d, 0x70, 0x73, 0x20, 0x6f, 0x76,
//                 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x7a, 0x79, 0x20, 0x64, 0x6f,
//                 0x67,
//             ]
//             .as_slice(),
//         );

//         let hash = sha.digest();

//         assert_eq!(
//             hash,
//             [
//                 0x07, 0xe5, 0x47, 0xd9, 0x58, 0x6f, 0x6a, 0x73, 0xf7, 0x3f, 0xba, 0xc0, 0x43, 0x5e,
//                 0xd7, 0x69, 0x51, 0x21, 0x8f, 0xb7, 0xd0, 0xc8, 0xd7, 0x88, 0xa3, 0x09, 0xd7, 0x85,
//                 0x43, 0x6b, 0xbb, 0x64, 0x2e, 0x93, 0xa2, 0x52, 0xa9, 0x54, 0xf2, 0x39, 0x12, 0x54,
//                 0x7d, 0x1e, 0x8a, 0x3b, 0x5e, 0xd6, 0xe1, 0xbf, 0xd7, 0x09, 0x78, 0x21, 0x23, 0x3f,
//                 0xa0, 0x53, 0x8f, 0x3d, 0xb8, 0x54, 0xfe, 0xe6
//             ]
//         );

//         // println!("sha512Stage2 took 0.000ms");
//         println!(
//             "sha512Stage2 took {}ms",
//             now.elapsed().as_micros() as f64 / 1000.0
//         );
//     }
// }
