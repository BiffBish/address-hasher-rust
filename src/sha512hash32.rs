// #![allow(arithmetic_overflow)]
use crate::{big_int::BigInt, curve, point::Point};
use crate::{
    profile, Profile, IS_PROFILE_RECONCILING, IS_PROFILING, PROFILING_DEPTH, PROFILING_MAP,
    PROFILING_PATH,
};
use colored::Colorize;
use std::hint::black_box;

const K: [u32; 160] = [
    1116352408, 3609767458, 1899447441, 602891725, 3049323471, 3964484399, 3921009573, 2173295548,
    961987163, 4081628472, 1508970993, 3053834265, 2453635748, 2937671579, 2870763221, 3664609560,
    3624381080, 2734883394, 310598401, 1164996542, 607225278, 1323610764, 1426881987, 3590304994,
    1925078388, 4068182383, 2162078206, 991336113, 2614888103, 633803317, 3248222580, 3479774868,
    3835390401, 2666613458, 4022224774, 944711139, 264347078, 2341262773, 604807628, 2007800933,
    770255983, 1495990901, 1249150122, 1856431235, 1555081692, 3175218132, 1996064986, 2198950837,
    2554220882, 3999719339, 2821834349, 766784016, 2952996808, 2566594879, 3210313671, 3203337956,
    3336571891, 1034457026, 3584528711, 2466948901, 113926993, 3758326383, 338241895, 168717936,
    666307205, 1188179964, 773529912, 1546045734, 1294757372, 1522805485, 1396182291, 2643833823,
    1695183700, 2343527390, 1986661051, 1014477480, 2177026350, 1206759142, 2456956037, 344077627,
    2730485921, 1290863460, 2820302411, 3158454273, 3259730800, 3505952657, 3345764771, 106217008,
    3516065817, 3606008344, 3600352804, 1432725776, 4094571909, 1467031594, 275423344, 851169720,
    430227734, 3100823752, 506948616, 1363258195, 659060556, 3750685593, 883997877, 3785050280,
    958139571, 3318307427, 1322822218, 3812723403, 1537002063, 2003034995, 1747873779, 3602036899,
    1955562222, 1575990012, 2024104815, 1125592928, 2227730452, 2716904306, 2361852424, 442776044,
    2428436474, 593698344, 2756734187, 3733110249, 3204031479, 2999351573, 3329325298, 3815920427,
    3391569614, 3928383900, 3515267271, 566280711, 3940187606, 3454069534, 4118630271, 4000239992,
    116418474, 1914138554, 174292421, 2731055270, 289380356, 3203993006, 460393269, 320620315,
    685471733, 587496836, 852142971, 1086792851, 1017036298, 365543100, 1126000580, 2618297676,
    1288033470, 3409855158, 1501505948, 4234509866, 1607167915, 987167468, 1816402316, 1246189591,
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
fn sigma0_32(x: u32, xl: u32) -> u32 {
    return (x >> 28 | (i32::wrapping_shl(xl as i32, 4) as u32))
        ^ ((xl >> 2) | (i32::wrapping_shl(x as i32, 30) as u32))
        ^ ((xl >> 7) | (i32::wrapping_shl(x as i32, 25) as u32));
}
fn sigma1_32(x: u32, xl: u32) -> i32 {
    return (((x >> 14) | (i32::wrapping_shl(xl as i32, 18)) as u32)
        ^ ((x >> 18) | (i32::wrapping_shl(xl as i32, 14)) as u32)
        ^ ((xl >> 9) | i32::wrapping_shl(x as i32, 23) as u32)) as i32;
}
fn gamma0_32(x: i32, xl: i32) -> i32 {
    return ((((x as u32) >> 1) | ((xl as u32) << 31))
        ^ (((x as u32) >> 8) | ((xl as u32) << 24))
        ^ ((x as u32) >> 7)) as i32;
}
fn gamma0l_32(x: i32, xl: i32) -> i32 {
    return ((((x as u32) >> 1) | ((xl as u32) << 31))
        ^ (((x as u32) >> 8) | ((xl as u32) << 24))
        ^ (((x as u32) >> 7) | ((xl as u32) << 25))) as i32;
}
fn gamma1_32(x: i32, xl: i32) -> i32 {
    return (((x as u32 >> 19) | ((xl as u32) << 13))
        ^ ((xl as u32 >> 29) | ((x as u32) << 3))
        ^ (x as u32 >> 6)) as i32;
}
fn gamma1l_32(x: i32, xl: i32) -> i32 {
    return ((((x as u32) >> 19) | ((xl as u32) << 13))
        ^ (((xl as u32) >> 29) | ((x as u32) << 3))
        ^ (((x as u32) >> 6) | ((xl as u32) << 26))) as i32;
}
fn get_carry_32(a: u32, b: u32) -> u32 {
    if (a) < (b) {
        return 1;
    } else {
        return 0;
    }
}

// fn printHex(prefix: &str, itt: impl Iterator<Item = u32>) {
//     print!("{}  ", prefix);

//     // prefix: ff, ff, ff, ff, ff, ff, ff, ff, ff
//     //         ff, ff, ff, ff, ff, ff, ff, ff, ff
//     let mut count = 0;
//     for i in itt {
//         if count % 4 == 0 && count != 0 {
//             let padding = iter::repeat(" ").take(prefix.len()).collect::<String>();
//             println!();
//             print!("{}  ", padding);
//         }

//         print!("{:08x}, ", i);
//         count += 1;
//     }
//     println!();
//     println!();
// }

// fn printHexi32(prefix: &str, itt: impl Iterator<Item = i32>) {
//     print!("{}  ", prefix);

//     // prefix: ff, ff, ff, ff, ff, ff, ff, ff, ff
//     //         ff, ff, ff, ff, ff, ff, ff, ff, ff
//     let mut count = 0;
//     for i in itt {
//         if count % 4 == 0 && count != 0 {
//             let padding = iter::repeat(" ").take(prefix.len()).collect::<String>();
//             println!();
//             print!("{}  ", padding);
//         }

//         print!("{:08x}, ", i);
//         count += 1;
//     }
//     println!();
//     println!();
// }

const BLOCK_SIZE: usize = 32;
const FINAL_SIZE: usize = 28;
#[derive(Debug, Clone)]
pub struct Sha512Hash {
    _block: [u32; BLOCK_SIZE],
    _len: usize,
    _w: [i32; 160],

    _ah: u32,
    _bh: u32,
    _ch: u32,
    _dh: u32,
    _eh: u32,
    _fh: u32,
    _gh: u32,
    _hh: u32,

    _al: u32,
    _bl: u32,
    _cl: u32,
    _dl: u32,
    _el: u32,
    _fl: u32,
    _gl: u32,
    _hl: u32,
    // _block: [u8; 128]
}

impl Sha512Hash {
    pub fn new() -> Sha512Hash {
        return Sha512Hash {
            _block: [0; BLOCK_SIZE],
            _len: 0,
            _w: [0; 160],

            _ah: 1779033703,
            _bh: 3144134277,
            _ch: 1013904242,
            _dh: 2773480762,
            _eh: 1359893119,
            _fh: 2600822924,
            _gh: 528734635,
            _hh: 1541459225,

            _al: 4089235720,
            _bl: 2227873595,
            _cl: 4271175723,
            _dl: 1595750129,
            _el: 2917565137,
            _fl: 725511199,
            _gl: 4215389547,
            _hl: 327033209,
        };
    }
    #[profile()]
    pub fn update(&mut self, data: &[u32]) {
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

    pub fn reset(&mut self) {
        self._block.fill(0);
        self._len = 0;
        self._ah = 1779033703;
        self._bh = 3144134277;
        self._ch = 1013904242;
        self._dh = 2773480762;
        self._eh = 1359893119;
        self._fh = 2600822924;
        self._gh = 528734635;
        self._hh = 1541459225;
        self._al = 4089235720;
        self._bl = 2227873595;
        self._cl = 4271175723;
        self._dl = 1595750129;
        self._el = 2917565137;
        self._fl = 725511199;
        self._gl = 4215389547;
        self._hl = 327033209;
    }

    pub fn digest(&mut self) -> Vec<u32> {
        let rem = self._len % BLOCK_SIZE;

        // printHex("_update self._block before", self._block.iter().cloned());
        // println!("rem: {} len: {}", rem, self._len);
        self._block[rem] = 0x80000000;

        // Fill the rest of the block with zeros
        for i in rem + 1..BLOCK_SIZE {
            self._block[i] = 0;
        }
        self._block[BLOCK_SIZE - 9] = 0;

        if rem >= FINAL_SIZE {
            self._update(self._block);
            self._block.fill(0);
        }
        let bits = 8 * 4 * self._len;

        if bits <= 0xff {
            self._block[BLOCK_SIZE - 1] = bits as u32
        } else if bits <= 0xffff {
            self._block[BLOCK_SIZE - 1] = bits as u32
        } else if bits <= 0xffffff {
            self._block[BLOCK_SIZE - 1] = bits as u32
        } else {
            self._block[BLOCK_SIZE - 1] = bits as u32
        }

        // printHex("_update self._block", self._block.iter().cloned());

        self._update(self._block);

        let data = vec![
            self._ah, self._al, self._bh, self._bl, self._ch, self._cl, self._dh, self._dl,
            self._eh, self._el, self._fh, self._fl, self._gh, self._gl, self._hh, self._hl,
        ];
        // println!("data: {:?}", data);

        return data;
    }
    #[profile()]
    pub fn _update(&mut self, m: [u32; 32]) {
        // printHex("m", m.iter().cloned());
        let mut w = self._w;
        for i in 0..32 {
            w[i] = m[i] as i32;
        }
        // printHexi32("w", w.iter().cloned());
        let mut xh: i32;
        let mut xl: i32;
        let mut vgamma0: i32;
        let mut vgamma0l: i32;
        let mut vgamma1: i32;
        let mut vgamma1l: i32;
        let mut wi7h: i32;
        let mut wi7l: i32;
        let mut wi16h: i32;
        let mut wi16l: i32;
        let mut wil: i32;
        let mut wih: i32;

        for i in (32..160).step_by(2) {
            xh = w[i - 30];
            xl = w[i - 30 + 1];
            vgamma0 = gamma0_32(xh, xl);
            vgamma0l = gamma0l_32(xl, xh);
            xh = w[i - 4];
            xl = w[i - 4 + 1];
            vgamma1 = gamma1_32(xh, xl);
            vgamma1l = gamma1l_32(xl, xh);
            wi7h = w[i - 14];
            wi7l = w[i - 14 + 1];
            wi16h = w[i - 32];
            wi16l = w[i - 32 + 1];
            wil = i32::wrapping_add(vgamma0l as i32, wi7l as i32);
            wih = i32::wrapping_add(
                i32::wrapping_add(vgamma0, wi7h),
                get_carry_32(wil as u32, vgamma0l as u32) as i32,
            ) | 0;
            wil = i32::wrapping_add(wil, vgamma1l) | 0;
            wih = i32::wrapping_add(
                wih,
                i32::wrapping_add(vgamma1, get_carry_32(wil as u32, vgamma1l as u32) as i32),
            ) | 0;
            wil = i32::wrapping_add(wil, wi16l) | 0;
            wih = i32::wrapping_add(
                wih,
                i32::wrapping_add(wi16h, get_carry_32(wil as u32, wi16l as u32) as i32),
            ) | 0;

            w[i] = wih as i32;
            w[i + 1] = wil as i32;
        }

        let ah = self._ah as i32;
        let bh = self._bh as i32;
        let ch = self._ch as i32;
        let dh = self._dh as i32;
        let eh = self._eh as i32;
        let fh = self._fh as i32;
        let gh = self._gh as i32;
        let hh = self._hh as i32;
        let al = self._al as i32;
        let bl = self._bl as i32;
        let cl = self._cl as i32;
        let dl = self._dl as i32;
        let el = self._el as i32;
        let fl = self._fl as i32;
        let gl = self._gl as i32;
        let hl = self._hl as i32;

        let mut kil: i32;
        let mut t1l: i32;
        let mut t1h: i32;
        let mut t2l: i32;
        let mut t2h: i32;

        let mut dr1 = ah;
        let mut dr2 = bh;
        let mut dr3 = ch;
        let mut dr4 = dh;
        let mut dr5 = eh;
        let mut dr6 = fh;
        let mut dr7 = gh;
        let mut dr8 = hh;
        let mut dr9 = al;
        let mut dr10 = bl;
        let mut dr11 = cl;
        let mut dr12 = dl;
        let mut dr13 = el;
        let mut dr14 = fl;
        let mut dr15 = gl;
        let mut dr0 = hl;

        let mut chl;
        let mut vsigma0l;

        let mut wil: i32;

        for j in (0..160).step_by(2) {
            // for (var j = 0; j < 160; j += 2) {
            kil = K[j + 1] as i32;
            t1l = i32::wrapping_add(dr0, sigma1_32(dr13 as u32, dr5 as u32) as i32);
            t1h = i32::wrapping_add(
                i32::wrapping_add(dr8, sigma1_32(dr5 as u32, dr13 as u32) as i32),
                get_carry_32(t1l as u32, dr0 as u32) as i32,
            );

            wil = w[j + 1] as i32;
            chl = cha(dr13, dr14, dr15);
            t1l = i32::wrapping_add(t1l, chl);
            t1h = i32::wrapping_add(
                t1h,
                i32::wrapping_add(
                    cha(dr5, dr6, dr7),
                    get_carry_32(t1l as u32, chl as u32) as i32,
                ),
            );
            t1l = i32::wrapping_add(t1l, kil);
            t1h = i32::wrapping_add(
                t1h,
                i32::wrapping_add(K[j] as i32, get_carry_32(t1l as u32, kil as u32) as i32),
            );
            t1l = i32::wrapping_add(t1l, wil);
            vsigma0l = sigma0_32(dr9 as u32, dr1 as u32) as i32;
            let majr = maj(dr9 as i32, dr10 as i32, dr11 as i32);

            t2l = i32::wrapping_add(vsigma0l as i32, majr as i32);
            t2h = i32::wrapping_add(
                i32::wrapping_add(
                    sigma0_32(dr1 as u32, dr9 as u32) as i32,
                    maj(dr1 as i32, dr2 as i32, dr3 as i32) as i32,
                ),
                get_carry_32(t2l as u32, vsigma0l as u32) as i32,
            );

            dr0 = dr15;
            dr15 = dr14;
            dr14 = dr13;
            dr13 = i32::wrapping_add(dr12, t1l);

            dr8 = dr7;
            dr7 = dr6;
            dr6 = dr5;

            t1h = i32::wrapping_add(
                t1h,
                i32::wrapping_add(w[j] as i32, get_carry_32(t1l as u32, wil as u32) as i32),
            );

            dr5 = i32::wrapping_add(
                i32::wrapping_add(dr4, t1h),
                get_carry_32(dr13 as u32, dr12 as u32) as i32,
            );

            dr12 = dr11;
            dr11 = dr10;
            dr10 = dr9;
            dr4 = dr3;
            dr3 = dr2;
            dr2 = dr1;

            dr9 = i32::wrapping_add(t1l, t2l);
            dr1 = i32::wrapping_add(
                i32::wrapping_add(t1h, t2h),
                get_carry_32(dr9 as u32, t1l as u32) as i32,
            );
        }

        self._al = i32::wrapping_add(self._al as i32, dr9) as u32;
        self._bl = i32::wrapping_add(self._bl as i32, dr10) as u32;
        self._cl = i32::wrapping_add(self._cl as i32, dr11) as u32;
        self._dl = i32::wrapping_add(self._dl as i32, dr12) as u32;
        self._el = i32::wrapping_add(self._el as i32, dr13) as u32;
        self._fl = i32::wrapping_add(self._fl as i32, dr14) as u32;
        self._gl = i32::wrapping_add(self._gl as i32, dr15) as u32;
        self._hl = i32::wrapping_add(self._hl as i32, dr0) as u32;
        self._ah = i32::wrapping_add(
            self._ah as i32,
            i32::wrapping_add(dr1, get_carry_32(self._al as u32, dr9 as u32) as i32),
        ) as u32;
        self._bh = i32::wrapping_add(
            self._bh as i32,
            i32::wrapping_add(dr2, get_carry_32(self._bl as u32, dr10 as u32) as i32),
        ) as u32;
        self._ch = i32::wrapping_add(
            self._ch as i32,
            i32::wrapping_add(dr3, get_carry_32(self._cl as u32, dr11 as u32) as i32),
        ) as u32;
        self._dh = i32::wrapping_add(
            self._dh as i32,
            i32::wrapping_add(dr4, get_carry_32(self._dl as u32, dr12 as u32) as i32),
        ) as u32;
        self._eh = i32::wrapping_add(
            self._eh as i32,
            i32::wrapping_add(dr5, get_carry_32(self._el as u32, dr13 as u32) as i32),
        ) as u32;
        self._fh = i32::wrapping_add(
            self._fh as i32,
            i32::wrapping_add(dr6, get_carry_32(self._fl as u32, dr14 as u32) as i32),
        ) as u32;
        self._gh = i32::wrapping_add(
            self._gh as i32,
            i32::wrapping_add(dr7, get_carry_32(self._gl as u32, dr15 as u32) as i32),
        ) as u32;
        self._hh = i32::wrapping_add(
            self._hh as i32,
            i32::wrapping_add(dr8, get_carry_32(self._hl as u32, dr0 as u32) as i32),
        ) as u32;

        // println!("al: {:x}", self._al);
        // println!("bl: {:x}", self._bl);
        // println!("cl: {:x}", self._cl);
        // println!("dl: {:x}", self._dl);
        // println!("el: {:x}", self._el);
        // println!("fl: {:x}", self._fl);
        // println!("gl: {:x}", self._gl);
        // println!("hl: {:x}", self._hl);
        // println!("ah: {:x}", self._ah);
        // println!("bh: {:x}", self._bh);
        // println!("ch: {:x}", self._ch);
        // println!("dh: {:x}", self._dh);
        // println!("eh: {:x}", self._eh);
        // println!("fh: {:x}", self._fh);
        // println!("gh: {:x}", self._gh);
        // println!("hh: {:x}", self._hh);
    }
    pub fn _hash(&self) -> [u32; 16] {
        let mut data = [0u32; 16];

        data[0] = self._ah;
        data[1] = self._al;
        data[2] = self._bh;
        data[3] = self._bl;
        data[4] = self._ch;
        data[5] = self._cl;
        data[6] = self._dh;
        data[7] = self._dl;
        data[8] = self._eh;
        data[9] = self._el;
        data[10] = self._fh;
        data[11] = self._fl;
        data[12] = self._gh;
        data[13] = self._gl;
        data[14] = self._hh;
        data[15] = self._hl;

        return data;
    }
}

#[cfg(test)]
mod tests {

    use std::time::Instant;

    use super::*;

    #[test]
    fn sha512_stage1() {
        let mut sha = Sha512Hash::new();
        sha._update([
            0x54686520, 0x71756963, 0x6b206272, 0x6f776e20, 0x666f7820, 0x6a756d70, 0x73206f76,
            0x65722074, 0x6865206c, 0x617a7920, 0x646f6700, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000,
        ]);

        let hash = sha._hash();

        assert_eq!(
            hash,
            [
                0xb2df56f0, 0x4c428ef0, 0x51c0c474, 0x6789e6f6, 0xa8f99687, 0xe9aa191b, 0xef404243,
                0x4021754b, 0x21e12ca6, 0x17999e85, 0x07157044, 0x70844fad, 0x793726ad, 0xc4f747a3,
                0x815de0c7, 0x56550ab3,
            ]
        );
    }

    #[test]
    fn sha512_stage2() {
        let now = Instant::now();

        let mut sha = Sha512Hash::new();
        sha.update(
            vec![
                0x54686520, 0x71756963, 0x6b206272, 0x6f776e20, 0x666f7820, 0x6a756d70, 0x73206f76,
                0x65722074, 0x6865206c, 0x617a7920, 0x646f6700,
            ]
            .as_slice(),
        );

        let hash = sha.digest();

        assert_eq!(
            hash,
            [
                0x962a42f5, 0xc9b8711e, 0x858a9b8b, 0x66e903a1, 0x3765d70b, 0xed7b306b, 0x3dc33941,
                0x9bd34ed7, 0x0d6e6a1b, 0xcfb5df13, 0x1d8772c1, 0x3ce5b054, 0x981a705e, 0xfebb048b,
                0xdd7c5f4b, 0x606dcfa5
            ]
        );

        // println!("sha512Stage2 took 0.000ms");
        println!(
            "sha512Stage2 took {}ms",
            now.elapsed().as_micros() as f64 / 1000.0
        );
    }
}
