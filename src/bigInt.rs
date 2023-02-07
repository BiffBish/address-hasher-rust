const DBITS: u32 = 26;

#[derive(Debug, Clone)]
pub struct BigInt {
    t: usize,
    s: i32,
    DB: u32,
    DM: u32,
    DV: u32,
    FV: u64,
    F1: u32,
    F2: u32,
    data: Vec<u32>,
}

// var rr,
//   vv,
//   BI_RM = "0123456789abcdefghijklmnopqrstuvwxyz",
//   BI_RC = new Array();
// for (rr = "0".charCodeAt(0), vv = 0; vv <= 9; ++vv) BI_RC[rr++] = vv;
// for (rr = "a".charCodeAt(0), vv = 10; vv < 36; ++vv) BI_RC[rr++] = vv;
// for (rr = "A".charCodeAt(0), vv = 10; vv < 36; ++vv) BI_RC[rr++] = vv;
// function intAt(b: string, a: any) {
//   var d = BI_RC[b.charCodeAt(a)];
//   return null == d ? -1 : d;
// }

const BI_RM: &str = "0123456789abcdefghijklmnopqrstuvwxyz";
const BI_RC: [u32; 36] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
];

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

fn intAt(b: &[u8], i: usize) -> u32 {
    let mut r = b[i] as u32;
    r <<= 8;
    r |= b[i + 1] as u32;
    r <<= 8;
    r |= b[i + 2] as u32;
    r <<= 8;
    r |= b[i + 3] as u32;
    r
}
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
            DB: DBITS,
            DM: (1 << DBITS) - 1,
            DV: 1 << DBITS,
            FV: 2u64.pow(52),
            F1: 52 - DBITS,
            F2: 2 * DBITS - 52,
            data: vec![n],
        }
    }

    pub fn new() -> BigInt {
        BigInt {
            t: 0,
            s: 0,
            DB: DBITS,
            DM: (1 << DBITS) - 1,
            DV: 1 << DBITS,
            FV: 2u64.pow(52),
            F1: 52 - DBITS,
            F2: 2 * DBITS - 52,
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

    pub fn new_from_iterator<'a, I: DoubleEndedIterator<Item = &'a u8>>(
        itt: I,
        radix: u32,
    ) -> BigInt {
        let mut n = BigInt::new();
        n.from_iterator(itt, radix);
        n
    }

    pub fn from_iterator<'a, I: DoubleEndedIterator<Item = &'a u8>>(
        &mut self,
        itt: I,
        radix: u32,
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
            } else if f + 8 > self.DB {
                self.data[self.t - 1] |= (bits & ((1 << (self.DB - f)) - 1)) << f;
                self.data.push(bits >> (self.DB - f));
                self.t += 1;
            } else {
                self.data[self.t - 1] |= bits << f;
            }
            // }
            f += 8;
            if f >= self.DB {
                f -= self.DB;
            }
        }

        self.data.push(0);
        self.clamp();
        self
    }

    pub fn from_hex(&mut self, s: &str) {
        let mut i = s.len();
        let mut k = 0;
        let mut is_neg = false;
        let mut r = 0;
        self.t = 0;
        self.s = 0;
        i -= 1;
        while i >= 0 {
            let c = s.chars().nth(i).unwrap();
            if c == '-' {
                is_neg = true;
                break;
            } else if c >= '0' && c <= '9' {
                r += (c as u32 - '0' as u32) << k;
                if k == 0 {
                    self.data.push(r);
                    self.t += 1;
                    r = 0;
                }
                k += 4;
                if k >= 8 {
                    k -= 8;
                }
            } else if c >= 'a' && c <= 'f' {
                r += (c as u32 - 'a' as u32 + 10) << k;
                if k == 0 {
                    self.data.push(r);
                    self.t += 1;
                    r = 0;
                }
                k += 4;
                if k >= 8 {
                    k -= 8;
                }
            } else if c >= 'A' && c <= 'F' {
                r += (c as u32 - 'A' as u32 + 10) << k;
                if k == 0 {
                    self.data.push(r);
                    self.t += 1;
                    r = 0;
                }
                k += 4;
                if k >= 8 {
                    k -= 8;
                }
            }
            i -= 1;
        }
        if k > 0 {
            self.data.push(r);
            self.t += 1;
        }
        if is_neg {
            self.s = -1;
        }
    }

    pub fn clamp(&mut self) {
        let a = self.s & self.DM as i32;
        while self.t > 0 && self.data[self.t - 1] == a as u32 {
            self.t -= 1;
        }
    }

    pub fn sub_to(&self, d: &BigInt, out: &mut BigInt) {
        let mut e = 0;
        let mut g = 0i32;
        let mut b = std::cmp::min(d.t, self.t);
        let max = std::cmp::max(d.t, self.t);
        // Make sure that out.data is big enough
        if out.data.len() < max {
            out.data.resize(max, 0);
        }

        for i in 0..b {
            g += (self.data[i] as i32 - d.data[i] as i32);
            out.data[i] = (g & self.DM as i32) as u32;
            g >>= self.DB;
        }
        e = b;

        if d.t < self.t {
            g -= d.s;
            for i in b..self.t {
                g += self.data[i] as i32;
                out.data[i] = (g & self.DM as i32) as u32;
                g >>= self.DB;
            }
            g += self.s;
            e = self.t;
        } else {
            g += self.s;

            for j in b..d.t {
                g -= d.data[j] as i32;
                out.data[j] = (g & self.DM as i32) as u32;
                g >>= self.DB;
            }
            g -= d.s;
            e = d.t;
        }
        out.s = if g < 0 { -1 } else { 0 };
        if g < -1 {
            if e > out.data.len() {
                panic!("out.data is too small");
            } else if e == out.data.len() {
                out.data.push(self.DV as u32);
            } else {
                out.data[e] = self.DV as u32;
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
    pub fn subtract_ip(&mut self, a: &BigInt) -> &mut BigInt {
        let mut r = BigInt::new();
        self.sub_to(a, &mut r);
        *self = r;
        self
    }

    pub fn add_to(&self, d: &BigInt, out: &mut BigInt) {
        let mut e = 0;
        let mut carry = 0i32;
        let m = std::cmp::min(d.t, self.t);
        let max = std::cmp::max(d.t, self.t);
        // Make sure that out.data is big enough

        if out.data.len() < max {
            out.data.resize(max, 0);
        }

        for i in 0..m {
            carry += (self.data[i] as i32 + d.data[i] as i32);
            out.data[i] = (carry & self.DM as i32) as u32;
            carry >>= self.DB;
        }

        if d.t < self.t {
            carry += d.s;
            for i in m..self.t {
                carry += self.data[i] as i32;
                out.data[i] = (carry & self.DM as i32) as u32;
                carry >>= self.DB;
            }
            e = self.t;
            carry += self.s;
        } else {
            carry += self.s;

            for e in m..d.t {
                carry += d.data[e] as i32;
                out.data[e] = (carry & self.DM as i32) as u32;
                carry >>= self.DB;
            }
            e = d.t;
            carry += d.s;
        }

        out.s = if carry < 0 { -1 } else { 0 };

        if carry < -1 {
            out.data[e] = self.DV + carry as u32;
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
    pub fn add_ip(&mut self, a: &BigInt) -> &mut BigInt {
        let mut r = BigInt::new();
        self.add_to(a, &mut r);
        *self = r;
        self
    }

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

    fn multiply_to(&self, c: &BigInt, e: &mut BigInt) {
        let mut b = self.abs();
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
        if (self.s != c.s) {
            ZERO.clone().sub_to(&e.clone(), e);
        };
    }
    pub fn multiply(&self, a: &BigInt) -> BigInt {
        let mut r = BigInt::new();
        self.multiply_to(a, &mut r);
        r
    }
    pub fn bit_length(&self) -> u32 {
        if self.t <= 0 {
            return 0;
        }
        return (self.DB * (self.t - 1 as usize) as u32)
            + nbits(self.data[self.t - 1] ^ (self.s & self.DM as i32) as u32);
    }
    pub fn test_bit(&self, n: u32) -> bool {
        let j = (n / self.DB) as usize;
        if j >= self.t {
            return self.s != 0;
        }
        return (self.data[j] & (1 << (n % self.DB))) != 0;
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

    pub fn modulo(&self, b: &BigInt) -> BigInt {
        // Might need to be nbi
        let mut c = BigInt::new();
        let abs = self.abs();
        abs.div_rem_to(&b, None, &mut c);
        if self.s < 0 && c.compareTo(&ZERO.clone()) > 0 {
            // c = b - c;
            b.sub_to(&c.clone(), &mut c);
        }
        return c;
    }
    pub fn mod_inverse(&self, m: &BigInt) -> BigInt {
        // var compIsEven = comparator.isEven();

        // const cV = comparator.clone();
        // const oV = this.clone();
        // const rV = nbv(1);
        // const iv1 = nbv(0);
        // const iv2 = nbv(0);
        // const iv3 = nbv(1);
        let compIsEven = m.isEven();
        if self.isEven() && compIsEven || m.signum() == 0 {
            return ZERO.clone();
        }
        let mut cV = m.clone();
        let mut oV = self.clone();
        let mut rV = BigInt::nvb(1);
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
        while cV.signum() != 0 {
            while cV.isEven() {
                cV.clone().r_shift_to(&1, &mut cV);
                if compIsEven {
                    if !rV.isEven() && !iv1.isEven() {
                        rV.clone().add_to(&self.clone(), &mut rV);
                        iv1.clone().sub_to(&m.clone(), &mut iv1);
                    }
                    rV.clone().r_shift_to(&1, &mut rV);
                } else {
                    if !iv1.isEven() {
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
            while oV.isEven() {
                oV.clone().r_shift_to(&1, &mut oV);
                if compIsEven {
                    if !(iv2.isEven() && iv3.isEven()) {
                        iv2.clone().add_to(&self.clone(), &mut iv2);
                        iv3.clone().sub_to(&m.clone(), &mut iv3);
                    }
                    iv2.clone().r_shift_to(&1, &mut iv2);
                } else {
                    if !iv3.isEven() {
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

            if cV.compareTo(&oV.clone()) >= 0 {
                cV.clone().sub_to(&oV.clone(), &mut cV);
                if compIsEven {
                    rV.clone().sub_to(&iv2.clone(), &mut rV);
                }
                iv1.clone().sub_to(&iv3.clone(), &mut iv1);
            } else {
                oV.clone().sub_to(&cV.clone(), &mut oV);
                if compIsEven {
                    iv2.clone().sub_to(&rV.clone(), &mut iv2);
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
        if oV.compareTo(&ONE.clone()) != 0 {
            return ZERO.clone();
        }
        if iv3.compareTo(&m.clone()) >= 0 {
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

    pub fn abs(&self) -> BigInt {
        if self.s < 0 {
            return self.negate();
        }
        return self.clone();
    }

    pub fn negate(&self) -> BigInt {
        let mut a = BigInt::new();
        ZERO.sub_to(self, &mut a);
        return a;
    }

    pub fn compareTo(&self, b: &BigInt) -> i32 {
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

    pub fn dlShiftTo(&self, c: &usize, b: &mut BigInt) {
        let mut a;
        // for (a = this.t - 1; a >= 0; --a) b.data[a + c] = this.data[a];
        // for (a = c - 1; a >= 0; --a) b.data[a] = 0;
        // (b.t = this.t + c), (b.s = this.s);

        if b.data.len() <= (self.t + c) {
            b.data.resize(self.t + c + 1, 0);
        }
        a = self.t - 1;

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
    pub fn drShiftTo(&self, c: &usize, b: &mut BigInt) {
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

    pub fn l_shift_to(&self, j: u32, e: &mut BigInt) {
        let mut d;
        let b = j % self.DB;
        let a = self.DB - b;
        let g = (1 << a) - 1;
        let f = (j / self.DB) as u32;
        let mut h = (self.s << b) & self.DM as i32;

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

        if (f > 0) {
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
    pub fn r_shift_to(&self, g: &u32, d: &mut BigInt) {
        d.s = self.s;
        let e = (g / self.DB) as usize;
        if e >= self.t {
            d.t = 0;
            return;
        }
        let b = g % self.DB;
        let a = self.DB - b;
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

    pub fn div_rem_to(&self, n: &BigInt, mut h: Option<BigInt>, g: &mut BigInt) {
        // let w = n.abs();
        // if (w.t <= 0) return;
        // let k = this.abs();
        // if (k.t < w.t) {
        // if (h != null) h.fromInt(0);
        // if (null != g) this.copyTo(g);
        // return;
        // }
        let mut w = n.abs();
        if w.t <= 0 {
            return;
        }
        let mut k = self.abs();
        if k.t < w.t {
            if h.is_some() {
                h.unwrap().from_hex("0");
            }
            // if g.is_some() {
            self.copy_to(g);
            // }
            return;
        }

        // if (null == g) g = nbi();
        // let d = nbi();
        // let a = this.s;
        // let l = n.s;
        // let v = this.DB - nbits(w.data[w.t - 1]);
        // if (v > 0) {
        // w.lShiftTo(v, d);
        // k.lShiftTo(v, g);
        // } else {
        // w.copyTo(d);
        // k.copyTo(g);
        // }

        let mut d = BigInt::nvb(0);
        let mut a = self.s;
        let mut l = n.s;
        let mut v = self.DB - nbits(w.data[w.t - 1]);
        if v > 0 {
            w.l_shift_to(v, &mut d);
            k.l_shift_to(v, g);
        } else {
            w.copy_to(&mut d);
            k.copy_to(g);
        }

        let p = d.t;
        let b = d.data[p - 1];
        if b == 0 {
            return;
        }
        let o = b as i128 * (1 << self.F1) as i128
            + (if p > 1 {
                d.data[p - 2] as i128 >> self.F2 as i128
            } else {
                0
            });
        let A = self.FV / o as u64;
        let z = (1 << self.F1) / o;
        let x = 1 << self.F2;
        let mut u = g.t;
        let s = u - p;

        let mut f = match h {
            Some(ref h) => h.clone(),
            None => BigInt::nvb(0),
        };

        d.dlShiftTo(&s, &mut f);
        if g.compareTo(&f) >= 0 {
            g.data[g.t] = 1;
            g.t += 1;
            // g = f + g;
            (g.clone()).sub_to(&f, g);
        }
        let one = ONE.clone();
        let mut c = 0i128;
        ONE.dlShiftTo(&p, &mut f);
        // d = f - d;
        f.sub_to(&(d.clone()), &mut d);

        if d.t < p {
            for i in d.t..p {
                d.data[i] = 0;
                d.t += 1;
            }
        }
        if (s > 0) {
            for i in (0..=s - 1).rev() {
                u -= 1;
                if (g.data.len() < u + 1) && g.data[u + 1] == b {
                    c = self.DM as i128
                } else {
                    c = ((g.data[u] * A as u32) as i128 + ((g.data[u - 1] + x) as i128 * z));
                }

                g.data[u] += d.am(0, c, g, i, 0, p);

                if g.data[u] < c as u32 {
                    d.dlShiftTo(&(i), &mut f);
                    // g += f;
                    (g.clone()).sub_to(&f, g);

                    while g.data[u] < c as u32 {
                        (g.clone()).sub_to(&f, g);
                        c -= 1;
                    }
                }
            }
        }

        // match h {
        //     Some(ref mut h) => {
        //         g.drShiftTo(&p, h);
        //         if a != l {
        //             ZERO.sub_to(h, h);
        //         }
        //     }
        //     None => {}
        // }

        // if h.is_some() {
        //     let mut h = h.unwrap();

        //     g.drShiftTo(&p, &mut h);
        //     if a != l {
        //         ZERO.sub_to(&h, &mut h);
        //         // BigInt::ZERO.sub_to();
        //     }
        // }

        if let Some(ref mut h) = h {
            g.drShiftTo(&p, h);
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

    pub fn isEven(&self) -> bool {
        (if self.t > 0 {
            self.data[0] & 1
        } else {
            self.s as u32
        }) == 0
    }

    pub fn toByteArray(&self) -> Vec<u8> {
        let mut bitNumber = self.t - 1;
        let mut r = Vec::new();
        r.push(self.s as u8);
        let mut p = self.DB - (self.t as u32 * self.DB) % 8;
        let mut faceBits = 0;
        let mut something = 0 as u32;
        if p < self.DB {
            faceBits = self.data[bitNumber] >> p;
            if faceBits != ((self.s as u32 & self.DM) >> p) {
                r.push(faceBits as u8);
                something += 1;
            }
        }
        bitNumber += 1;
        while bitNumber > 0 {
            if p < 8 {
                faceBits = (self.data[bitNumber - 1] & ((1 << p) - 1)) << (8 - p);
                p += self.DB - 8;
                bitNumber -= 1;
                faceBits |= self.data[bitNumber - 1] >> p;
            } else {
                p -= 8;
                faceBits = (self.data[bitNumber - 1] >> p) & 0xff;
                if p <= 0 {
                    p += self.DB;
                    bitNumber -= 1;
                }
            }
            if (faceBits & 0b00000000_10000000) != 0 {
                faceBits |= 0b11111111_00000000;
            }
            if something == 0 {
                if ((self.s & 0b00000000_10000000) as u32 != (faceBits & 0b00000000_10000000)) {
                    something += 1;
                }
            }
            if something > 0 || faceBits != self.s as u32 {
                r.push(faceBits as u8);
                something += 1;
            }
        }
        r
    }

    pub fn to_byte_array_unsigned(&self) -> Vec<u8> {
        let byte_array = self.toByteArray();
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
            if d.data[iat] >= a.DV {
                d.data[iat] -= a.DV;
                d.data[iat + 1] = 1;
            }
        }
        let c = a.t - 1;

        d.data[d.t - 1] += a.am(c as u32, a.data[c] as i128, d, 2 * c, 0, 1);
        d.s = 0;
        d.clamp();
    }

    pub fn exp(&self, h: u32, j: &mut BigInt) -> BigInt {
        if h > 4294967295 || h < 1 {
            return ONE.clone();
        }
        let mut f = BigInt::new();
        let mut a = BigInt::new();
        let mut d = self.clone();
        // let mut idfk =
        let mut c = nbits(h) - 1;
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
