// #![allow(arithmetic_overflow)]

const K: [u32; 64] = [
    1116352408, 1899447441, 3049323471, 3921009573, 961987163, 1508970993, 2453635748, 2870763221,
    3624381080, 310598401, 607225278, 1426881987, 1925078388, 2162078206, 2614888103, 3248222580,
    3835390401, 4022224774, 264347078, 604807628, 770255983, 1249150122, 1555081692, 1996064986,
    2554220882, 2821834349, 2952996808, 3210313671, 3336571891, 3584528711, 113926993, 338241895,
    666307205, 773529912, 1294757372, 1396182291, 1695183700, 1986661051, 2177026350, 2456956037,
    2730485921, 2820302411, 3259730800, 3345764771, 3516065817, 3600352804, 4094571909, 275423344,
    430227734, 506948616, 659060556, 883997877, 958139571, 1322822218, 1537002063, 1747873779,
    1955562222, 2024104815, 2227730452, 2361852424, 2428436474, 2756734187, 3204031479, 3329325298,
];

fn cha<T: std::ops::BitXor<Output = T> + std::ops::BitAnd<Output = T> + Copy>(
    x: T,
    y: T,
    z: T,
) -> T {
    return z ^ (x & (y ^ z));
}
fn maj(x: i32, y: i32, z: i32) -> i32 {
    return (x & y) | (z & (x | y));
}

// fn sigma0_32(x: u32, xl: u32) -> u32 {
//     return (x >> 28 | (i32::wrapping_shl(xl as i32, 4) as u32))
//         ^ ((xl >> 2) | (i32::wrapping_shl(x as i32, 30) as u32))
//         ^ ((xl >> 7) | (i32::wrapping_shl(x as i32, 25) as u32));
// }

fn sigma0(x: u32) -> u32 {
    return ((x >> 2) | i32::wrapping_shl(x as i32, 30) as u32)
        ^ ((x >> 13) | i32::wrapping_shl(x as i32, 19) as u32)
        ^ ((x >> 22) | i32::wrapping_shl(x as i32, 10) as u32);
}
fn sigma1(x: u32) -> u32 {
    //   return (
    //     ((x >>> 6) | (x << 26)) ^ ((x >>> 11) | (x << 21)) ^ ((x >>> 25) | (x << 7))
    //   );
    // }
    return ((x >> 6) | i32::wrapping_shl(x as i32, 26) as u32)
        ^ ((x >> 11) | i32::wrapping_shl(x as i32, 21) as u32)
        ^ ((x >> 25) | i32::wrapping_shl(x as i32, 7) as u32);
}
fn gamma0(x: u32) -> u32 {
    //   return ((x >>> 7) | (x << 25)) ^ ((x >>> 18) | (x << 14)) ^ (x >>> 3);
    return ((x >> 7) | i32::wrapping_shl(x as i32, 25) as u32)
        ^ ((x >> 18) | i32::wrapping_shl(x as i32, 14) as u32)
        ^ (x >> 3);
}

const BLOCK_SIZE: usize = 64;
const FINAL_SIZE: usize = 56;

pub struct Sha256Hash {
    _block: [u8; BLOCK_SIZE],
    _len: usize,
    _w: [i32; 160],

    _a: u32,
    _b: u32,
    _c: u32,
    _d: u32,
    _e: u32,
    _f: u32,
    _g: u32,
    _h: u32,
    // _block: [u8; 128]
}

impl Sha256Hash {
    pub fn new() -> Sha256Hash {
        return Sha256Hash {
            _block: [0; BLOCK_SIZE],
            _len: 0,
            _w: [0; 160],

            _a: 1779033703,
            _b: 3144134277,
            _c: 1013904242,
            _d: 2773480762,
            _e: 1359893119,
            _f: 2600822924,
            _g: 528734635,
            _h: 1541459225,
        };
    }

    pub fn update(&mut self, data: &[u8]) {
        let length = data.len();
        let mut accum = self._len;
        let mut offset = 0;
        while offset < length {
            let assigned = accum % BLOCK_SIZE;
            let remainder = std::cmp::min(length - offset, BLOCK_SIZE - assigned);
            for i in 0..remainder {
                self._block[assigned + i] = data[offset + i];
            }
            offset += remainder;
            accum += remainder;
            if accum % BLOCK_SIZE == 0 {
                self._update(self._block);
            }
        }
        self._len += length;
    }

    // pub fn reset(&mut self) {
    //     self._block.fill(0);
    //     self._len = 0;
    //     self._a = 1779033703;
    //     self._b = 3144134277;
    //     self._c = 1013904242;
    //     self._d = 2773480762;
    //     self._e = 1359893119;
    //     self._f = 2600822924;
    //     self._g = 528734635;
    //     self._h = 1541459225;
    // }

    pub fn digest(&mut self) -> Vec<u8> {
        let rem = self._len % BLOCK_SIZE;
        self._block[rem] = 128;
        // Fill the rest of the block with zeros
        for i in rem + 1..BLOCK_SIZE {
            self._block[i] = 0;
        }
        self._block[BLOCK_SIZE - 9] = 0;

        if rem >= FINAL_SIZE {
            self._update(self._block);
            self._block.fill(0);
        }
        let bits = 8 * self._len;

        if bits <= 0xff {
            self._block[BLOCK_SIZE - 1] = (bits & 255) as u8;
        } else if bits <= 0xffff {
            self._block[BLOCK_SIZE - 2] = ((bits >> 8) & 255) as u8;
            self._block[BLOCK_SIZE - 1] = (bits & 255) as u8;
        } else if bits <= 0xffffff {
            self._block[BLOCK_SIZE - 3] = ((bits >> 16) & 255) as u8;
            self._block[BLOCK_SIZE - 2] = ((bits >> 8) & 255) as u8;
            self._block[BLOCK_SIZE - 1] = (bits & 255) as u8;
        } else if bits <= 0xffffffff {
            self._block[BLOCK_SIZE - 4] = ((bits >> 24) & 255) as u8;
            self._block[BLOCK_SIZE - 3] = ((bits >> 16) & 255) as u8;
            self._block[BLOCK_SIZE - 2] = ((bits >> 8) & 255) as u8;
            self._block[BLOCK_SIZE - 1] = (bits & 255) as u8;
        } else if bits <= 0xffffffffff {
            self._block[BLOCK_SIZE - 5] = ((bits >> 32) & 255) as u8;
            self._block[BLOCK_SIZE - 4] = ((bits >> 24) & 255) as u8;
            self._block[BLOCK_SIZE - 3] = ((bits >> 16) & 255) as u8;
            self._block[BLOCK_SIZE - 2] = ((bits >> 8) & 255) as u8;
            self._block[BLOCK_SIZE - 1] = (bits & 255) as u8;
        }

        self._update(self._block);
        let mut data = vec![0u8; 32];

        fn write_u32_be(data: &mut [u8], offset: usize, value: u32) {
            data[offset] = (value >> 24) as u8;
            data[offset + 1] = (value >> 16) as u8;
            data[offset + 2] = (value >> 8) as u8;
            data[offset + 3] = value as u8;
        }

        write_u32_be(&mut data, 0, self._a);
        write_u32_be(&mut data, 4, self._b);
        write_u32_be(&mut data, 8, self._c);
        write_u32_be(&mut data, 12, self._d);
        write_u32_be(&mut data, 16, self._e);
        write_u32_be(&mut data, 20, self._f);
        write_u32_be(&mut data, 24, self._g);
        write_u32_be(&mut data, 28, self._h);
        return data;
    }

    pub fn _update(&mut self, m: [u8; BLOCK_SIZE]) {
        let mut w = self._w;
        for i in (0..16).step_by(2) {
            // for (; i < 32; i += 2) {
            w[i] = i32::from_be_bytes([m[4 * i], m[4 * i + 1], m[4 * i + 2], m[4 * i + 3]]);
            w[i + 1] = i32::from_be_bytes([m[4 * i + 4], m[4 * i + 5], m[4 * i + 6], m[4 * i + 7]]);
        }

        // for (; i < 64; ++i) {
        // let x = W[i - 2];
        // let a = ((x >>> 17) | (x << 15)) ^ ((x >>> 19) | (x << 13)) ^ (x >>> 10);
        // W[i] = 0 | (a + W[i - 7] + gamma0(W[i - 15]) + W[i - 16]);
        // }
        for i in 16..64 {
            let x = w[i - 2];
            let a = ((x as u32 >> 17) | (i32::wrapping_shl(x, 15)) as u32)
                ^ ((x as u32 >> 19) | i32::wrapping_shl(x, 13) as u32)
                ^ (x as u32 >> 10);

            let b = u32::wrapping_add(
                w[i - 7] as u32,
                u32::wrapping_add(gamma0(w[i - 15] as u32), w[i - 16] as u32),
            );

            w[i] = u32::wrapping_add(a as u32, b) as i32;
            // lowercase hex padded to 8 digits
        }

        let mut a = self._a as u32;
        let mut b = self._b as u32;
        let mut c = self._c as u32;
        let mut d = self._d as u32;
        let mut e = self._e as u32;
        let mut f = self._f as u32;
        let mut g = self._g as u32;
        let mut h = self._h as u32;

        for j in 0..64 {
            let t1 = u32::wrapping_add(
                u32::wrapping_add(
                    u32::wrapping_add(h as u32, sigma1(e as u32)),
                    cha(e as u32, f as u32, g as u32),
                ),
                u32::wrapping_add(K[j], w[j] as u32),
            );
            let t2 = u32::wrapping_add(sigma0(a as u32), maj(a as i32, b as i32, c as i32) as u32);
            h = g;
            g = f;
            f = e;
            e = u32::wrapping_add(d, t1);
            d = c;
            c = b;
            b = a;
            a = u32::wrapping_add(t1, t2);
        }

        self._a = u32::wrapping_add(self._a, a);
        self._b = u32::wrapping_add(self._b, b);
        self._c = u32::wrapping_add(self._c, c);
        self._d = u32::wrapping_add(self._d, d);
        self._e = u32::wrapping_add(self._e, e);
        self._f = u32::wrapping_add(self._f, f);
        self._g = u32::wrapping_add(self._g, g);
        self._h = u32::wrapping_add(self._h, h);
    }
    pub fn _hash(&self) -> [u8; 64] {
        let mut data = [0u8; 64];

        fn write_u32_be(data: &mut [u8], offset: usize, value: u32) {
            data[offset] = (value >> 24) as u8;
            data[offset + 1] = (value >> 16) as u8;
            data[offset + 2] = (value >> 8) as u8;
            data[offset + 3] = value as u8;
        }

        write_u32_be(&mut data, 0, self._a);
        write_u32_be(&mut data, 4, self._b);
        write_u32_be(&mut data, 8, self._c);
        write_u32_be(&mut data, 12, self._d);
        write_u32_be(&mut data, 16, self._e);
        write_u32_be(&mut data, 20, self._f);
        write_u32_be(&mut data, 24, self._g);
        write_u32_be(&mut data, 28, self._h);

        return data;
    }
}

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn sha512_stage1() {
//         let now = Instant::now();

//         let mut sha = Sha256Hash::new();
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

//         let mut sha = Sha256Hash::new();
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
