use core::panic;
const DBITS: u32 = 26;
use crate::{
    profile, Profile, IS_PROFILE_RECONCILING, IS_PROFILING, PROFILING_DEPTH, PROFILING_MAP,
    PROFILING_PATH,
};
use colored::Colorize;
use std::hint::black_box;

#[derive(Debug, Clone)]
pub struct BigInt {
    t: usize,
    s: i32,
    db: u32,
    dm: u32,
    dv: u32,
    fv: u64,
    f1: u32,
    f2: u32,
    data: Vec<u32>,
}

// const BI_RM: &str = "0123456789abcdefghijklmnopqrstuvwxyz";
// const BI_RC: [u32; 36] = [
//     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
//     26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
// ];

fn nbits(a: u32) -> u32 {
    let mut r = 1;
    let mut t = a;
    if (t & 0xffff0000) != 0 {
        t >>= 16;
        r += 16;
    }
    if (t & 0xff00) != 0 {
        t >>= 8;
        r += 8;
    }
    if (t & 0xf0) != 0 {
        t >>= 4;
        r += 4;
    }
    if (t & 0xc) != 0 {
        t >>= 2;
        r += 2;
    }
    if (t & 0x2) != 0 {
        r += 1;
    }
    r
}

// fn intAt(b: &[u8], i: usize) -> u32 {
//     let mut r = b[i] as u32;
//     r <<= 8;
//     r |= b[i + 1] as u32;
//     r <<= 8;
//     r |= b[i + 2] as u32;
//     r <<= 8;
//     r |= b[i + 3] as u32;
//     r
// }
// lazy_static! {
//     pub static ref ZERO: BigInt = BigInt::nvb(0);
//     pub static ref ONE: BigInt = BigInt::nvb(1);
//     pub static ref THREE: BigInt = BigInt::nvb(3);
// }

pub static ZERO: once_cell::sync::Lazy<BigInt> = once_cell::sync::Lazy::new(|| BigInt::nvb(0));
pub static ONE: once_cell::sync::Lazy<BigInt> = once_cell::sync::Lazy::new(|| BigInt::nvb(1));
pub static THREE: once_cell::sync::Lazy<BigInt> = once_cell::sync::Lazy::new(|| BigInt::nvb(3));

impl BigInt {
    pub fn nvb(n: u32) -> BigInt {
        BigInt {
            // t: 1,
            t: if n == 0 { 0 } else { 1 },
            s: 0,
            db: DBITS,
            dm: (1 << DBITS) - 1,
            dv: 1 << DBITS,
            fv: 2u64.pow(52),
            f1: 52 - DBITS,
            f2: 2 * DBITS - 52,
            data: vec![n],
        }
    }

    pub fn new() -> BigInt {
        BigInt {
            t: 0,
            s: 0,
            db: DBITS,
            dm: (1 << DBITS) - 1,
            dv: 1 << DBITS,
            fv: 2u64.pow(52),
            f1: 52 - DBITS,
            f2: 2 * DBITS - 52,
            data: Vec::new(),
        }
    }

    // // pub fn from_signed_bytes_be(b: &[u8]) {
    //     let mut i = b.len();
    //     let mut k = 0;
    //     let mut is_neg = false;
    //     let mut r = 0;

    //     let mut n = BigInt::new();
    //     n.t = 0;
    //     n.s = 0;
    //     i -= 1;
    //     while i >= 0 {
    //         let c = b[i];
    //         if c == '-' as u8 {
    //             is_neg = true;
    //             break;
    //         } else if c >= '0' as u8 && c <= '9' as u8 {
    //             r += (c as u32 - '0' as u32) << k;
    //             if k == 0 {
    //                 n.data.push(r);
    //                 n.t += 1;
    //                 r = 0;
    //             }
    //             k += 4;
    //             if k >= 8 {
    //                 k -= 8;
    //             }
    //         }
    //     }
    //     if k > 0 {
    //         n.data.push(r);
    //         n.t += 1;
    //     }
    //     if is_neg {
    //         n.s = -1;
    //     }
    // }

    pub fn new_from_iterator<'a, I: DoubleEndedIterator<Item = &'a u8>>(itt: I) -> BigInt {
        let mut n = BigInt::new();
        n.from_iterator(itt);
        n
    }

    pub fn from_iterator<'a, I: DoubleEndedIterator<Item = &'a u8>>(
        &mut self,
        itt: I,
    ) -> &mut BigInt {
        // Get the first char

        self.t = 0;
        self.s = 0;
        let mut f = 0;
        let mut bits;
        for i in itt.rev() {
            bits = (i & 0xff) as u32;

            // if bits == '-' as u32 {
            // } else {
            if f == 0 {
                self.data.push(bits as u32);
                self.t += 1;
            } else if f + 8 > self.db {
                self.data[self.t - 1] |= (bits & ((1 << (self.db - f)) - 1)) << f;
                self.data.push(bits >> (self.db - f));
                self.t += 1;
            } else {
                self.data[self.t - 1] |= bits << f;
            }
            // }
            f += 8;
            if f >= self.db {
                f -= self.db;
            }
        }

        self.data.push(0);
        self.clamp();
        self
    }

    pub fn from_hex(&mut self, _: &str) {
        panic!("Not implemented");
    }

    #[profile()]
    pub fn clamp(&mut self) {
        let a = self.s & self.dm as i32;
        while self.t > 0 && self.data[self.t - 1] == a as u32 {
            self.t -= 1;
        }
    }

    pub fn sub_to(&self, d: &BigInt, out: &mut BigInt) {
        let mut e;
        let mut g = 0i32;
        let b = std::cmp::min(d.t, self.t);
        let max = std::cmp::max(d.t, self.t);
        if out.data.len() < max {
            out.data.resize(max, 0);
        }

        for i in 0..b {
            g += self.data[i] as i32 - d.data[i] as i32;
            out.data[i] = (g & self.dm as i32) as u32;
            g >>= self.db;
        }

        if d.t < self.t {
            g -= d.s;
            for i in b..self.t {
                g += self.data[i] as i32;
                out.data[i] = (g & self.dm as i32) as u32;
                g >>= self.db;
            }
            g += self.s;
            e = self.t;
        } else {
            g += self.s;

            for j in b..d.t {
                g -= d.data[j] as i32;
                out.data[j] = (g & self.dm as i32) as u32;
                g >>= self.db;
            }
            g -= d.s;
            e = d.t;
        }
        out.s = if g < 0 { -1 } else { 0 };
        if g < -1 {
            if e > out.data.len() {
                panic!("out.data is too small");
            } else if e == out.data.len() {
                out.data.push(self.dv as u32);
            } else {
                out.data[e] = self.dv as u32;
            }
            e += 1;
        } else if g > 0 {
            if e > out.data.len() {
                panic!("out.data is too small");
            } else if e == out.data.len() {
                out.data.push(g as u32);
            } else {
                out.data[e] = g as u32;
            }

            e += 1;
        }
        out.t = e;
        out.clamp();
    }
    pub fn subtract(&self, a: &BigInt) -> BigInt {
        let mut r = BigInt::new();
        self.sub_to(a, &mut r);
        r
    }
    #[profile()]
    pub fn subtract_ip(&mut self, d: &BigInt) -> &mut BigInt {
        let mut e;
        let mut g = 0i32;
        let b = std::cmp::min(d.t, self.t);
        let max = std::cmp::max(d.t, self.t);
        if self.data.len() < max {
            self.data.resize(max, 0);
        }

        for i in 0..b {
            g += self.data[i] as i32 - d.data[i] as i32;
            self.data[i] = (g & self.dm as i32) as u32;
            g >>= self.db;
        }

        if d.t < self.t {
            g -= d.s;
            for i in b..self.t {
                g += self.data[i] as i32;
                self.data[i] = (g & self.dm as i32) as u32;
                g >>= self.db;
            }
            g += self.s;
            e = self.t;
        } else {
            g += self.s;

            for j in b..d.t {
                g -= d.data[j] as i32;
                self.data[j] = (g & self.dm as i32) as u32;
                g >>= self.db;
            }
            g -= d.s;
            e = d.t;
        }
        self.s = if g < 0 { -1 } else { 0 };
        if g < -1 {
            if e > self.data.len() {
                panic!("self.data is too small");
            } else if e == self.data.len() {
                self.data.push(self.dv as u32);
            } else {
                self.data[e] = self.dv as u32;
            }
            e += 1;
        } else if g > 0 {
            if e > self.data.len() {
                panic!("self.data is too small");
            } else if e == self.data.len() {
                self.data.push(g as u32);
            } else {
                self.data[e] = g as u32;
            }

            e += 1;
        }
        self.t = e;
        self.clamp();
        self
    }

    pub fn add_to(&self, d: &BigInt, out: &mut BigInt) {
        let mut e;
        let mut carry = 0i32;
        let m = std::cmp::min(d.t, self.t);
        let max = std::cmp::max(d.t, self.t);
        // Make sure that out.data is big enough

        if out.data.len() < max {
            out.data.resize(max, 0);
        }

        for i in 0..m {
            carry += self.data[i] as i32 + d.data[i] as i32;
            out.data[i] = (carry & self.dm as i32) as u32;
            carry >>= self.db;
        }

        if d.t < self.t {
            carry += d.s;
            for i in m..self.t {
                carry += self.data[i] as i32;
                out.data[i] = (carry & self.dm as i32) as u32;
                carry >>= self.db;
            }
            e = self.t;
            carry += self.s;
        } else {
            carry += self.s;

            for e in m..d.t {
                carry += d.data[e] as i32;
                out.data[e] = (carry & self.dm as i32) as u32;
                carry >>= self.db;
            }
            e = d.t;
            carry += d.s;
        }

        out.s = if carry < 0 { -1 } else { 0 };

        if carry < -1 {
            out.data[e] = self.dv + carry as u32;
            e += 1;
        } else if carry > 0 {
            if e > out.data.len() {
                panic!("out.data is too small");
            } else if e == out.data.len() {
                out.data.push(carry as u32);
            } else {
                out.data[e] = carry as u32;
            }

            e += 1;
        }
        out.t = e;
        out.clamp();
    }
    pub fn add(&self, a: &BigInt) -> BigInt {
        let mut r = BigInt::new();
        self.add_to(a, &mut r);
        r
    }
    pub fn add_ip(&mut self, other: &BigInt) -> &mut BigInt {
        let mut e;
        let mut carry = 0i32;
        let m = std::cmp::min(other.t, self.t);
        let max = std::cmp::max(other.t, self.t);
        // Make sure that out.data is big enough

        if self.data.len() < max {
            self.data.resize(max, 0);
        }

        for i in 0..m {
            carry += self.data[i] as i32 + other.data[i] as i32;
            self.data[i] = (carry & self.dm as i32) as u32;
            carry >>= self.db;
        }

        if other.t < self.t {
            carry += other.s;
            for i in m..self.t {
                carry += self.data[i] as i32;
                self.data[i] = (carry & self.dm as i32) as u32;
                carry >>= self.db;
            }
            e = self.t;
            carry += self.s;
        } else {
            carry += self.s;

            for e in m..other.t {
                carry += other.data[e] as i32;
                self.data[e] = (carry & self.dm as i32) as u32;
                carry >>= self.db;
            }
            e = other.t;
            carry += other.s;
        }

        self.s = if carry < 0 { -1 } else { 0 };

        if carry < -1 {
            self.data[e] = self.dv + carry as u32;
            e += 1;
        } else if carry > 0 {
            if e > self.data.len() {
                panic!("self.data is too small");
            } else if e == self.data.len() {
                self.data.push(carry as u32);
            } else {
                self.data[e] = carry as u32;
            }

            e += 1;
        }
        self.t = e;
        self.clamp();
        self
    }

    #[profile()]
    pub fn am(&mut self, i: u32, x: i128, w: &mut BigInt, j: usize, c: u32, n: usize) -> u32 {
        let mut c = c;
        let mut i = i;
        let mut j = j;
        for _ in (1..=n).rev() {
            let v = x * self.data[i as usize] as i128 + w.data[j as usize] as i128 + c as i128;
            c = (v as i128 / 67108864) as u32;
            w.data[j as usize] = 67108863 & v as u32;
            i += 1;
            j += 1;
        }
        c
    }

    #[profile()]
    fn multiply_to(&self, c: &BigInt, e: &mut BigInt) {
        let mut b = self.abs();
        let c_s = c.s;
        let f = c.abs();
        // let d =

        e.t = b.t + f.t;
        e.data = vec![0; e.t as usize];
        for i in 0..b.t {
            e.data[i] = 0;
        }
        for i in 0..f.t {
            e.data[i + b.t] = b.am(0, f.data[i] as i128, e, i, 0, b.t);
        }
        e.s = 0;
        e.clamp();
        if self.s != c_s {
            e.negate_ip();
        }
    }

    #[profile()]
    pub fn multiply(&self, a: &BigInt) -> BigInt {
        let mut r = BigInt::new();
        self.multiply_to(a, &mut r);
        r
    }

    #[profile()]
    pub fn multiply_ip(&mut self, c: &BigInt) -> &mut BigInt {
        let s = self.s;
        let mut b = self.abs();
        let f = c.abs();
        self.t = b.t + f.t;
        self.data = vec![0; self.t as usize];
        for i in 0..b.t {
            self.data[i] = 0;
        }
        for i in 0..f.t {
            self.data[i + b.t] = b.am(0, f.data[i] as i128, self, i, 0, b.t);
        }
        self.s = 0;
        self.clamp();
        if s != c.s {
            self.negate_ip();
        };
        self
    }

    pub fn bit_length(&self) -> u32 {
        if self.t <= 0 {
            return 0;
        }
        return (self.db * (self.t - 1 as usize) as u32)
            + nbits(self.data[self.t - 1] ^ (self.s & self.dm as i32) as u32);
    }
    pub fn test_bit(&self, n: u32) -> bool {
        let j = (n / self.db) as usize;
        if j >= self.t {
            return self.s != 0;
        }
        return (self.data[j] & (1 << (n % self.db))) != 0;
    }
    pub fn signum(&self) -> i32 {
        if self.s < 0 {
            -1
        } else if self.t <= 0 || (self.t == 1 && self.data[0] <= 0) {
            0
        } else {
            1
        }
    }

    pub fn modulo(&self, x: &BigInt) -> BigInt {
        // Might need to be nbi
        let mut c = BigInt::new();
        self.abs().div_rem_to(&x, None, &mut c);
        if self.s < 0 && c.compare_to(&ZERO.clone()) > 0 {
            // c = b - c;
            x.sub_to(&c.clone(), &mut c);
        }
        return c;
    }

    #[profile()]
    pub fn modulo_ip(&mut self, x: &BigInt) -> &mut BigInt {
        let s = self.s;
        self.abs_ip();
        self.div_rem_ip(&x, None);
        if s < 0 && self.compare_to(&ZERO.clone()) > 0 {
            let mut e;
            let mut g = 0i32;
            let b = std::cmp::min(self.t, x.t);
            let max = std::cmp::max(self.t, x.t);
            if self.data.len() < max {
                self.data.resize(max, 0);
            }

            for i in 0..b {
                g += x.data[i] as i32 - self.data[i] as i32;
                self.data[i] = (g & x.dm as i32) as u32;
                g >>= x.db;
            }

            if self.t < x.t {
                g -= self.s;
                for i in b..x.t {
                    g += x.data[i] as i32;
                    self.data[i] = (g & x.dm as i32) as u32;
                    g >>= x.db;
                }
                g += x.s;
                e = x.t;
            } else {
                g += x.s;

                for j in b..self.t {
                    g -= self.data[j] as i32;
                    self.data[j] = (g & x.dm as i32) as u32;
                    g >>= x.db;
                }
                g -= self.s;
                e = self.t;
            }
            self.s = if g < 0 { -1 } else { 0 };
            if g < -1 {
                if e > self.data.len() {
                    panic!("self.data is too small");
                } else if e == self.data.len() {
                    self.data.push(x.dv as u32);
                } else {
                    self.data[e] = x.dv as u32;
                }
                e += 1;
            } else if g > 0 {
                if e > self.data.len() {
                    panic!("self.data is too small");
                } else if e == self.data.len() {
                    self.data.push(g as u32);
                } else {
                    self.data[e] = g as u32;
                }

                e += 1;
            }
            self.t = e;
            self.clamp();
        }
        self
    }

    #[profile()]
    pub fn mod_inverse(&self, m: &BigInt) -> BigInt {
        // var compIsEven = comparator.isEven();

        // const cV = comparator.clone();
        // const oV = this.clone();
        // const rV = nbv(1);
        // const iv1 = nbv(0);
        // const iv2 = nbv(0);
        // const iv3 = nbv(1);
        let comp_is_even = m.is_even();
        if self.is_even() && comp_is_even || m.signum() == 0 {
            return ZERO.clone();
        }
        let mut c_v = m.clone();
        let mut o_v = self.clone();
        let mut r_v = BigInt::nvb(1);
        let mut iv1 = BigInt::nvb(0);
        let mut iv2 = BigInt::nvb(0);
        let mut iv3 = BigInt::nvb(1);

        // for (; 0 != cV.signum(); ) {
        //   for (; cV.isEven(); ) {
        //     cV.rShiftTo(1, cV);
        //     if (compIsEven) {
        //       if (!(rV.isEven() && iv1.isEven())) {
        //         rV.addTo(this, rV);
        //         iv1.subTo(comparator, iv1);
        //       }
        //       rV.rShiftTo(1, rV);
        //     } else {
        //       iv1.isEven() || iv1.subTo(comparator, iv1);
        //     }
        //     iv1.rShiftTo(1, iv1);
        //   }
        while c_v.signum() != 0 {
            while c_v.is_even() {
                c_v.clone().r_shift_to(&1, &mut c_v);
                if comp_is_even {
                    if !r_v.is_even() && !iv1.is_even() {
                        r_v.clone().add_to(&self.clone(), &mut r_v);
                        iv1.clone().sub_to(&m.clone(), &mut iv1);
                    }
                    r_v.clone().r_shift_to(&1, &mut r_v);
                } else {
                    if !iv1.is_even() {
                        iv1.clone().sub_to(&m.clone(), &mut iv1);
                    }
                }
                iv1.clone().r_shift_to(&1, &mut iv1);
            }
            //   for (; oV.isEven(); ) {
            //     oV.rShiftTo(1, oV);
            //     if (compIsEven) {
            //       if (!(iv2.isEven() && iv3.isEven())) {
            //         iv2.addTo(this, iv2);
            //         iv3.subTo(comparator, iv3);
            //       }
            //       iv2.rShiftTo(1, iv2);
            //     } else {
            //       iv3.isEven() || iv3.subTo(comparator, iv3);
            //     }
            //     iv3.rShiftTo(1, iv3);
            //   }
            while o_v.is_even() {
                o_v.clone().r_shift_to(&1, &mut o_v);
                if comp_is_even {
                    if !(iv2.is_even() && iv3.is_even()) {
                        iv2.clone().add_to(&self.clone(), &mut iv2);
                        iv3.clone().sub_to(&m.clone(), &mut iv3);
                    }
                    iv2.clone().r_shift_to(&1, &mut iv2);
                } else {
                    if !iv3.is_even() {
                        iv3.clone().sub_to(&m.clone(), &mut iv3);
                    }
                }
                iv3.clone().r_shift_to(&1, &mut iv3);
            }
            //   if (cV.compareTo(oV) >= 0) {
            //     cV.subTo(oV, cV);
            //     if (compIsEven) {
            //       rV.subTo(iv2, rV);
            //     }
            //     iv1.subTo(iv3, iv1);
            //   } else {
            //     oV.subTo(cV, oV);
            //     let hResult = oV.toString(2);
            //     if (compIsEven) {
            //       iv2.subTo(rV, iv2);
            //     }
            //     iv3.subTo(iv1, iv3);
            //   }
            // }

            if c_v.compare_to(&o_v.clone()) >= 0 {
                c_v.clone().sub_to(&o_v.clone(), &mut c_v);
                if comp_is_even {
                    r_v.clone().sub_to(&iv2.clone(), &mut r_v);
                }
                iv1.clone().sub_to(&iv3.clone(), &mut iv1);
            } else {
                o_v.clone().sub_to(&c_v.clone(), &mut o_v);
                if comp_is_even {
                    iv2.clone().sub_to(&r_v.clone(), &mut iv2);
                }
                iv3.clone().sub_to(&iv1.clone(), &mut iv3);
            }
        }

        // if (0 != oV.compareTo(BigI.ONE)) return BigI.ZERO;
        // if (iv3.compareTo(comparator) >= 0) return iv3.subtract(comparator);
        // if (iv3.signum() < 0) {
        //   iv3.addTo(comparator, iv3);
        //   if (iv3.signum() < 0) {
        //     return iv3.add(comparator);
        //   } else {
        //     return iv3;
        //   }
        // }
        // return iv3;
        if o_v.compare_to(&ONE.clone()) != 0 {
            return ZERO.clone();
        }
        if iv3.compare_to(&m.clone()) >= 0 {
            return iv3.subtract(&m.clone());
        }
        if iv3.signum() < 0 {
            iv3.clone().add_to(&m.clone(), &mut iv3);
            if iv3.signum() < 0 {
                return iv3.add(&m.clone());
            } else {
                return iv3;
            }
        }
        return iv3;
    }

    #[profile()]
    pub fn abs(&self) -> BigInt {
        if self.s < 0 {
            return self.negate();
        }
        return self.clone();
    }
    #[profile()]
    pub fn abs_ip(&mut self) -> &mut BigInt {
        if self.s < 0 {
            return self.negate_ip();
        }
        self
    }

    #[profile()]
    pub fn negate(&self) -> BigInt {
        let mut a = BigInt::new();
        ZERO.sub_to(self, &mut a);
        return a;
    }

    #[profile()]
    pub fn negate_ip(&mut self) -> &mut BigInt {
        ZERO.sub_to(&self.clone(), self);
        return self;
    }

    pub fn compare_to(&self, b: &BigInt) -> i32 {
        let mut d = self.s - b.s;
        if 0 != d {
            return d;
        }
        let c = self.t;
        d = c as i32 - b.t as i32;
        if 0 != d {
            return if self.s < 0 { -d } else { d };
        }

        for i in (0..=c - 1).rev() {
            let d = self.data[i] as i32 - b.data[i] as i32;
            if d != 0 {
                return d as i32;
            }
        }

        return 0;
    }

    #[profile()]
    pub fn dl_shift_to(&self, c: &usize, b: &mut BigInt) {
        if b.data.len() <= (self.t + c) {
            b.data.resize(self.t + c + 1, 0);
        }
        let mut a = self.t - 1;

        for i in (0..a + 1).rev() {
            b.data[i + c] = self.data[i];
        }

        if c > &0 {
            a = c - 1;
            for i in (0..a + 1).rev() {
                b.data[i] = 0;
            }
        }

        b.t = self.t + c;
        b.s = self.s;
    }
    pub fn dr_shift_to(&self, c: &usize, b: &mut BigInt) {
        let mut a;
        // for (a = c; a < this.t; ++a) b.data[a - c] = this.data[a];
        // (b.t = Math.max(this.t - c, 0)), (b.s = this.s);
        a = c.clone();
        while a < self.t {
            b.data[a - c] = self.data[a];
            a += 1;
        }
        b.t = std::cmp::max(self.t - c, 0);
        b.s = self.s;
    }
    #[profile()]
    pub fn l_shift_to(&self, j: u32, e: &mut BigInt) {
        let mut d;
        let b = j % self.db;
        let a = self.db - b;
        let g = (1 << a) - 1;
        let f = (j / self.db) as u32;
        let mut h = (self.s << b) & self.dm as i32;

        d = self.t as i32 - 1;

        if e.data.len() < (d + f as i32 + 2) as usize {
            e.data.resize((d + f as i32 + 2) as usize, 0);
        }
        e.data[f as usize] = h as u32;

        if d >= 0 {
            for i in (0..d + 1).rev() {
                e.data[(i + f as i32 + 1) as usize] = (self.data[i as usize] >> a) | h as u32;
                h = ((self.data[i as usize] & g) << b) as i32;
            }
        }

        if f > 0 {
            d = f as i32 - 1;
            while d >= 0 {
                e.data[d as usize] = 0;
                if d == 0 {
                    break;
                }
                d -= 1;
            }
        }

        e.data[f as usize] = h as u32;
        e.t = self.t + f as usize + 1;
        e.s = self.s;
        e.clamp();
    }

    #[profile()]
    pub fn l_shift_ip(&mut self, j: u32) {
        // let mut e = self.clone();
        let mut d;
        let b = j % self.db;
        let a = self.db - b;
        let g = (1 << a) - 1;
        let f = (j / self.db) as u32;
        let mut h = (self.s << b) & self.dm as i32;
        d = self.t as i32 - 1;

        if self.data.len() < (d + f as i32 + 2) as usize {
            // panic!("l_shift_ip: data too small");
            self.data.resize((d + f as i32 + 2) as usize, 0);
        }

        if d >= 0 {
            for i in (0..d + 1).rev() {
                self.data[(i + f as i32 + 1) as usize] = (self.data[i as usize] >> a) | h as u32;
                h = ((self.data[i as usize] & g) << b) as i32;
            }
        }
        self.data[f as usize] = h as u32;
        self.t = self.t + f as usize + 1;
        self.s = self.s;
        self.clamp();
    }

    #[profile()]
    pub fn r_shift_to(&self, g: &u32, d: &mut BigInt) {
        d.s = self.s;
        let e = (g / self.db) as usize;
        if e >= self.t {
            d.t = 0;
            return;
        }
        let b = g % self.db;
        let a = self.db - b;
        let f = (1 << b) - 1;
        d.data[0] = self.data[e] >> b;

        for i in (e + 1)..self.t {
            d.data[i - e - 1] |= (self.data[i] & f) << a;
            d.data[i - e] = self.data[i] >> b;
        }
        if b > 0 {
            d.data[self.t - e - 1] |= (self.s as u32 & f) << a;
        }
        d.t = self.t - e;
        d.clamp();
    }

    #[profile()]
    pub fn r_shift_ip(&mut self, g: &u32) {
        let e = (g / self.db) as usize;
        if e >= self.t {
            self.t = 0;
            return;
        }
        let b = g % self.db;
        let a = self.db - b;
        let f = (1 << b) - 1;
        self.data[0] = self.data[e] >> b;

        for i in (e + 1)..self.t {
            self.data[i - e - 1] |= (self.data[i] & f) << a;
            self.data[i - e] = self.data[i] >> b;
        }
        if b > 0 {
            self.data[self.t - e - 1] |= (self.s as u32 & f) << a;
        }
        self.t -= e;
        self.clamp();
    }

    pub fn shift_left(&self, b: i32) -> BigInt {
        let mut a = BigInt::nvb(0);
        if b < 0 {
            self.r_shift_to(&(b.abs() as u32), &mut a);
        } else {
            self.l_shift_to(b as u32, &mut a);
        }
        return a;
    }

    pub fn copy_to(&self, b: &mut BigInt) {
        b.data = self.data.clone();
        b.t = self.t;
        b.s = self.s;
    }

    #[profile()]
    pub fn div_rem_to(&self, n: &BigInt, mut h: Option<BigInt>, g: &mut BigInt) {
        let w = n.abs();
        let k = self.abs();
        let mut d = BigInt::nvb(0);
        let a = self.s;
        let l = n.s;
        let v = self.db - nbits(w.data[w.t - 1]);
        w.l_shift_to(v, &mut d);
        k.l_shift_to(v, g);
        let p = d.t;
        let b = d.data[p - 1];
        let o = b as i128 * (1 << self.f1) as i128
            + (if p > 1 {
                d.data[p - 2] as i128 >> self.f2 as i128
            } else {
                0
            });
        let a_a = self.fv / o as u64;
        let z = (1 << self.f1) / o;
        let x = 1 << self.f2;
        let mut u = g.t;
        let s = u - p;

        let mut f = match h {
            Some(ref h) => h.clone(),
            None => BigInt::nvb(0),
        };

        d.dl_shift_to(&s, &mut f);
        if g.compare_to(&f) >= 0 {
            g.data[g.t] = 1;
            g.t += 1;
            // g = f + g;
            (g.clone()).sub_to(&f, g);
        }
        let mut c;
        ONE.dl_shift_to(&p, &mut f);
        // d = f - d;
        f.sub_to(&(d.clone()), &mut d);

        if d.t < p {
            for i in d.t..p {
                d.data[i] = 0;
                d.t += 1;
            }
        }
        if s > 0 {
            for i in (0..=s - 1).rev() {
                u -= 1;
                if (g.data.len() < u + 1) && g.data[u + 1] == b {
                    c = self.dm as i128
                } else {
                    c = (g.data[u] * a_a as u32) as i128 + ((g.data[u - 1] + x) as i128 * z);
                }

                g.data[u] += d.am(0, c, g, i, 0, p);

                if g.data[u] < c as u32 {
                    d.dl_shift_to(&(i), &mut f);
                    // g += f;
                    (g.clone()).sub_to(&f, g);

                    while g.data[u] < c as u32 {
                        (g.clone()).sub_to(&f, g);
                        c -= 1;
                    }
                }
            }
        }

        if let Some(ref mut h) = h {
            g.dr_shift_to(&p, h);
            if a != l {
                ZERO.sub_to(&h.clone(), h);
            }
        }

        g.t = p;
        g.clamp();
        if v > 0 {
            (g.clone()).r_shift_to(&v, g);
        }
        if a < 0 {
            ZERO.sub_to(&g.clone(), g);
        }
    }

    #[profile()]
    pub fn div_rem_ip(&mut self, n: &BigInt, mut h: Option<BigInt>) -> &mut BigInt {
        let l = n.s;
        let w = n.abs();

        let mut d = BigInt::nvb(0);
        let a = self.s;
        let v = self.db - nbits(w.data[w.t - 1]);
        // if v > 0 {
        w.l_shift_to(v, &mut d);
        self.abs_ip();
        self.l_shift_ip(v);

        let p = d.t;
        let b = d.data[p - 1];
        let o = b as i128 * (1 << self.f1) as i128
            + (if p > 1 {
                d.data[p - 2] as i128 >> self.f2 as i128
            } else {
                0
            });
        let a_a = self.fv / o as u64;
        let z = (1 << self.f1) / o;
        let x = 1 << self.f2;
        let mut u = self.t;
        let s = u - p;

        let mut f = match h {
            Some(ref h) => h.clone(),
            None => BigInt::nvb(0),
        };

        d.dl_shift_to(&s, &mut f);
        if self.compare_to(&f) >= 0 {
            self.data[self.t] = 1;
            self.t += 1;
            // self = f + self;
            self.subtract_ip(&f);
        }
        let mut c;
        ONE.dl_shift_to(&p, &mut f);
        // d = f - d;
        f.sub_to(&(d.clone()), &mut d);

        if d.t < p {
            for i in d.t..p {
                d.data[i] = 0;
                d.t += 1;
            }
        }
        if s > 0 {
            for i in (0..=s - 1).rev() {
                u -= 1;
                if (self.data.len() < u + 1) && self.data[u + 1] == b {
                    c = self.dm as i128
                } else {
                    c = (self.data[u] * a_a as u32) as i128 + ((self.data[u - 1] + x) as i128 * z);
                }

                self.data[u] += d.am(0, c, self, i, 0, p);

                if self.data[u] < c as u32 {
                    d.dl_shift_to(&(i), &mut f);
                    self.subtract_ip(&f);
                    while self.data[u] < c as u32 {
                        self.subtract_ip(&f);
                        c -= 1;
                    }
                }
            }
        }

        if let Some(ref mut h) = h {
            self.dr_shift_to(&p, h);
            if a != l {
                h.negate_ip();
            }
        }

        self.t = p;
        self.clamp();
        if v > 0 {
            self.r_shift_ip(&v);
        }
        if a < 0 {
            self.negate_ip();
        }
        self
    }

    pub fn is_even(&self) -> bool {
        (if self.t > 0 {
            self.data[0] & 1
        } else {
            self.s as u32
        }) == 0
    }

    pub fn to_byte_array(&self) -> Vec<u8> {
        let mut bit_number = self.t - 1;
        let mut r = Vec::new();
        r.push(self.s as u8);
        let mut p = self.db - (self.t as u32 * self.db) % 8;
        let mut face_bits;
        let mut something = 0 as u32;
        if p < self.db {
            face_bits = self.data[bit_number] >> p;
            if face_bits != ((self.s as u32 & self.dm) >> p) {
                r.push(face_bits as u8);
                something += 1;
            }
        }
        bit_number += 1;
        while bit_number > 0 {
            if p < 8 {
                face_bits = (self.data[bit_number - 1] & ((1 << p) - 1)) << (8 - p);
                p += self.db - 8;
                bit_number -= 1;
                face_bits |= self.data[bit_number - 1] >> p;
            } else {
                p -= 8;
                face_bits = (self.data[bit_number - 1] >> p) & 0xff;
                if p <= 0 {
                    p += self.db;
                    bit_number -= 1;
                }
            }
            if (face_bits & 0b00000000_10000000) != 0 {
                face_bits |= 0b11111111_00000000;
            }
            if something == 0 {
                if (self.s & 0b00000000_10000000) as u32 != (face_bits & 0b00000000_10000000) {
                    something += 1;
                }
            }
            if something > 0 || face_bits != self.s as u32 {
                r.push(face_bits as u8);
                something += 1;
            }
        }
        r
    }

    pub fn to_byte_array_unsigned(&self) -> Vec<u8> {
        let byte_array = self.to_byte_array();
        if byte_array[0] == 0 {
            byte_array[1..].to_vec()
        } else {
            byte_array
        }
    }

    pub fn square(&self) -> BigInt {
        let mut a = BigInt::new();
        self.square_to(&mut a);
        a
    }
    pub fn square_to(&self, d: &mut BigInt) {
        let mut a = self.abs();
        d.t = 2 * a.t;

        d.data = vec![0; d.t as usize];
        for b in 0..d.t {
            d.data[b] = 0;
        }

        for i in 0..(a.t - 1) {
            let iat = i + a.t;
            let e = a.am(i as u32, a.data[i] as i128, d, 2 * i, 0, 1);
            d.data[iat] += a.am(
                i as u32 + 1,
                2 * a.data[i] as i128,
                d,
                2 * i + 1,
                e,
                a.t - i - 1,
            );
            if d.data[iat] >= a.dv {
                d.data[iat] -= a.dv;
                d.data[iat + 1] = 1;
            }
        }
        let c = a.t - 1;

        d.data[d.t - 1] += a.am(c as u32, a.data[c] as i128, d, 2 * c, 0, 1);
        d.s = 0;
        d.clamp();
    }

    pub fn exp(&self, h: u32, _: &mut BigInt) -> BigInt {
        if h < 1 {
            return ONE.clone();
        }
        let mut f = BigInt::new();
        let mut a = BigInt::new();
        let d = self.clone();
        let c = nbits(h) - 1;
        d.copy_to(&mut f);
        for i in (0..=c - 1).rev() {
            f.square_to(&mut a);
            if (h & (1 << i)) > 0 {
                a.multiply_to(&d, &mut f);
            } else {
                let temp = f;
                f = a;
                a = temp;
            }
        }
        f
    }

    pub fn pow(&self, a: u32) -> BigInt {
        self.exp(a, &mut BigInt::new())
    }
}
