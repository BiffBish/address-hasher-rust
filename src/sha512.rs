use std::thread::sleep;
use std::time::{Duration, Instant};

// #![allow(arithmetic_overflow)]

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
    let aa = x as u32 >> 28;
    let bb = i32::wrapping_shl(xl as i32, 4);
    let cc = x as u32 >> 2;
    let dd = i32::wrapping_shl(x as i32, 30);
    let ee = xl as u32 >> 7;
    let ff = i32::wrapping_shl(x as i32, 25);

    return (x >> 28 | (i32::wrapping_shl(xl as i32, 4) as u32))
        ^ ((xl >> 2) | (i32::wrapping_shl(x as i32, 30) as u32))
        ^ ((xl >> 7) | (i32::wrapping_shl(x as i32, 25) as u32));
}
fn sigma1_32(x: u32, xl: u32) -> i32 {
    let aa = x as u32 >> 14;
    let bb = i32::wrapping_shl(xl as i32, 18);
    let cc = x as u32 >> 18;
    let dd = i32::wrapping_shl(xl as i32, 14);
    let ee = x as u32 >> 9;
    let ff = i32::wrapping_shl(x as i32, 23);

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

fn print_bits(debug: &str, x: i64) {
    // print out the debug message padded to 20 characters
    print!("{:20}", debug);

    // Print out the bits of a 64-bit integer in big-endian order
    for i in 0..64 {
        if (x >> (63 - i)) & 1 == 1 {
            print!("1");
        } else {
            print!("0");
        }
        if i % 32 == 31 {
            print!(" ");
        }
    }
    println!("");
}

fn print_bits_32(debug: &str, x: i32) {
    // print out the debug message padded to 20 characters
    print!("{:20}", debug);
    // Add a bunch of zeros to the front of the string
    print!("{:0>32}", "");
    print!(" ");
    // Print out the bits of a 64-bit integer in big-endian order
    for i in 0..32 {
        if (x >> (31 - i)) & 1 == 1 {
            print!("1");
        } else {
            print!("0");
        }
        if i % 32 == 31 {
            print!(" ");
        }
    }
    println!("");
}

fn print_hex(x: i32) {
    print!("{:x}", x);
}

struct Sha512 {
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

impl Sha512 {
    fn new() -> Sha512 {
        return Sha512 {
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

    fn _update(&mut self, M: [u8; 128]) {
        let mut W = self._w;

        for i in (0..32).step_by(2) {
            // for (; i < 32; i += 2) {
            W[i] = i32::from_be_bytes([M[4 * i], M[4 * i + 1], M[4 * i + 2], M[4 * i + 3]]);
            W[i + 1] = i32::from_be_bytes([M[4 * i + 4], M[4 * i + 5], M[4 * i + 6], M[4 * i + 7]]);
        }
        let mut xh: i32;
        let mut xl: i32;
        let mut vgamma0: i32;
        let mut vgamma0l: i32;
        let mut vgamma1: i32;
        let mut vgamma1l: i32;
        let mut Wi7h: i32;
        let mut Wi7l: i32;
        let mut Wi16h: i32;
        let mut Wi16l: i32;
        let mut Wil: i32;
        let mut Wih: i32;

        for i in (32..160).step_by(2) {
            xh = W[i - 30];
            xl = W[i - 30 + 1];
            vgamma0 = gamma0_32(xh, xl);
            vgamma0l = gamma0l_32(xl, xh);
            xh = W[i - 4];
            xl = W[i - 4 + 1];
            vgamma1 = gamma1_32(xh, xl);
            vgamma1l = gamma1l_32(xl, xh);
            Wi7h = W[i - 14];
            Wi7l = W[i - 14 + 1];
            Wi16h = W[i - 32];
            Wi16l = W[i - 32 + 1];
            Wil = i32::wrapping_add(vgamma0l as i32, Wi7l as i32);
            Wih = i32::wrapping_add(
                i32::wrapping_add(vgamma0, Wi7h),
                get_carry_32(Wil as u32, vgamma0l as u32) as i32,
            ) | 0;
            Wil = i32::wrapping_add(Wil, vgamma1l) | 0;
            Wih = i32::wrapping_add(
                Wih,
                i32::wrapping_add(vgamma1, get_carry_32(Wil as u32, vgamma1l as u32) as i32),
            ) | 0;
            Wil = i32::wrapping_add(Wil, Wi16l) | 0;
            Wih = i32::wrapping_add(
                Wih,
                i32::wrapping_add(Wi16h, get_carry_32(Wil as u32, Wi16l as u32) as i32),
            ) | 0;

            W[i] = Wih as i32;
            W[i + 1] = Wil as i32;
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

        let mut Wil: i32;

        for j in (0..160).step_by(2) {
            // for (var j = 0; j < 160; j += 2) {
            kil = K[j + 1] as i32;
            t1l = i32::wrapping_add(dr0, sigma1_32(dr13 as u32, dr5 as u32) as i32);
            t1h = i32::wrapping_add(
                i32::wrapping_add(dr8, sigma1_32(dr5 as u32, dr13 as u32) as i32),
                get_carry_32(t1l as u32, dr0 as u32) as i32,
            );

            Wil = W[j + 1] as i32;
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
            t1l = i32::wrapping_add(t1l, Wil);
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
                i32::wrapping_add(W[j] as i32, get_carry_32(t1l as u32, Wil as u32) as i32),
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
    }

    fn _hash(mut self) -> [u8; 64] {
        let mut data = [0u8; 64];

        fn write_u32_be(data: &mut [u8], offset: usize, value: u32) {
            data[offset] = (value >> 24) as u8;
            data[offset + 1] = (value >> 16) as u8;
            data[offset + 2] = (value >> 8) as u8;
            data[offset + 3] = value as u8;
        }

        write_u32_be(&mut data, 0, self._ah);
        write_u32_be(&mut data, 4, self._bh);
        write_u32_be(&mut data, 8, self._ch);
        write_u32_be(&mut data, 12, self._dh);
        write_u32_be(&mut data, 16, self._eh);
        write_u32_be(&mut data, 20, self._fh);
        write_u32_be(&mut data, 24, self._gh);
        write_u32_be(&mut data, 28, self._hh);
        write_u32_be(&mut data, 32, self._al);
        write_u32_be(&mut data, 36, self._bl);
        write_u32_be(&mut data, 40, self._cl);
        write_u32_be(&mut data, 44, self._dl);
        write_u32_be(&mut data, 48, self._el);
        write_u32_be(&mut data, 52, self._fl);
        write_u32_be(&mut data, 56, self._gl);
        write_u32_be(&mut data, 60, self._hl);

        return data;
    }
}

pub fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha512Stage1() {
        let now = Instant::now();

        for i in (0..1) {
            let mut sha = Sha512::new();
            sha._update([
                0x45, 0x43, 0x44, 0x44, 0x59, 0x43, 0x58, 0x52, 0x16, 0x5b, 0x5f, 0x45, 0x45, 0x16,
                0x58, 0x59, 0x5b, 0x5f, 0x58, 0x53, 0x53, 0x16, 0x52, 0x44, 0x53, 0x57, 0x5b, 0x16,
                0x51, 0x57, 0x46, 0x16, 0x55, 0x44, 0x59, 0x45, 0x45, 0x16, 0x57, 0x45, 0x45, 0x57,
                0x43, 0x5a, 0x42, 0x16, 0x42, 0x5e, 0x57, 0x58, 0x5d, 0x16, 0x55, 0x57, 0x46, 0x42,
                0x57, 0x5f, 0x58, 0x16, 0x46, 0x44, 0x59, 0x45, 0x46, 0x53, 0x44, 0x16, 0x52, 0x44,
                0x59, 0x46, 0x16, 0x52, 0x43, 0x42, 0x4f, 0x16, 0x51, 0x44, 0x59, 0x43, 0x46, 0x16,
                0x55, 0x57, 0x58, 0x52, 0x4f, 0x16, 0x41, 0x53, 0x57, 0x5a, 0x42, 0x5e, 0x16, 0x41,
                0x53, 0x57, 0x42, 0x5e, 0x53, 0x44, 0x16, 0x45, 0x55, 0x57, 0x5a, 0x53, 0x16, 0x46,
                0x43, 0x42, 0x36, 0x36, 0x36, 0x36, 0x36, 0x36, 0x36, 0x36, 0x36, 0x36, 0x36, 0x36,
                0x36, 0x36,
            ]);
            sha._hash();
        }
        println!(
            "Time elapsed in expensive_function() is: {:?}",
            now.elapsed()
        );
    }
}
