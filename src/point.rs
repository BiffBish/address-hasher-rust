use crate::{
    profile, Profile, IS_PROFILE_RECONCILING, IS_PROFILING, PROFILING_DEPTH, PROFILING_MAP,
    PROFILING_PATH,
};
use colored::Colorize;
use std::hint::black_box;

use crate::{
    big_int::{BigInt, THREE},
    curve::SECP251K1P,
    SECP256K1,
};
#[derive(Clone, Debug)]
pub struct Point {
    x: BigInt,
    y: BigInt,
    z: BigInt,
    zinv: Option<BigInt>,
}

impl Point {
    pub fn new(x: BigInt, y: BigInt, z: BigInt) -> Point {
        Point {
            x,
            y,
            zinv: None,
            z,
        }
    }
    pub fn from_affine(x: BigInt, y: BigInt) -> Point {
        let z = crate::big_int::ONE.clone();

        let point = Point {
            x: x,
            y: y,
            zinv: None,
            z: z,
        };
        point
    }
    pub fn multiply(&self, k: &BigInt) -> Point {
        point_multiply(self, k)
    }

    pub fn affine_x(&self) -> BigInt {
        let binding = self.z.mod_inverse(&SECP256K1.p);
        let zinv = match &self.zinv {
            Some(zinv) => zinv,
            None => &binding,
        };

        // return this.x.multiply(this.zInv).mod(this.curve.p);
        self.x.multiply(zinv).modulo(&SECP256K1.p)
    }

    pub fn affine_y(&self) -> BigInt {
        let binding = self.z.mod_inverse(&SECP256K1.p);
        let zinv = match &self.zinv {
            Some(zinv) => zinv,
            None => &binding,
        };
        self.y.multiply(zinv).modulo(&SECP256K1.p)
    }

    pub fn get_encoded(&self) -> Vec<u8> {
        let x = self.affine_x();
        let y = self.affine_y();

        let mut buffer = vec![0; 33];
        buffer[0] = if y.is_even() { 2 } else { 3 };
        let mut byte_array = x.to_byte_array_unsigned();

        if byte_array.len() < 32 {
            let mut i = 0;
            for _ in byte_array.len()..32 {
                byte_array.push(0);
            }
        }
        // println!("byte_array.len(): {:?}", byte_array.len());
        buffer[1..].copy_from_slice(&byte_array[byte_array.len() - 32..]);
        return buffer;
    }
    pub fn add(&self, a: &Point) -> Point {
        let x1 = &self.x;
        let y1 = &self.y;
        let x2 = &a.x;
        let u =
            a.y.multiply(&self.z)
                .subtract(&y1.multiply(&a.z))
                .modulo(&SECP256K1.p);
        let v = x2
            .multiply(&self.z)
            .subtract(&x1.multiply(&a.z))
            .modulo(&SECP256K1.p);

        let v2 = v.square();
        let v3 = v2.multiply(&v);
        let x1v2 = x1.multiply(&v2);
        let zu2 = u.square().multiply(&self.z);
        let x3 = zu2
            .subtract(&x1v2.shift_left(1))
            .multiply(&a.z)
            .subtract(&v3)
            .multiply(&v)
            .modulo(&SECP256K1.p);
        let y3 = x1v2
            .multiply(&THREE)
            .multiply(&u)
            .subtract(&y1.multiply(&v3))
            .subtract(&zu2.multiply(&u))
            .multiply(&a.z)
            .add(&u.multiply(&v3))
            .modulo(&SECP256K1.p);
        let z3 = v3.multiply(&self.z).multiply(&a.z).modulo(&SECP256K1.p);
        return Point {
            x: x3,
            y: y3,
            zinv: None,
            z: z3,
        };
    }

    pub fn add_ip(&mut self, a: &Point) -> &mut Point {
        point_add_ip(self, a);
        self
    }

    pub fn negate(&self) -> Point {
        let y = SECP256K1.p.subtract(&self.y);
        return Point {
            x: self.x.clone(),
            y,
            zinv: self.zinv.clone(),
            z: self.z.clone(),
        };
    }
    pub fn twice(&self) -> Point {
        let x1 = &self.x;
        let y1 = &self.y;
        let y1z1a = y1.multiply(&self.z);
        let y1z1 = y1z1a.modulo(&SECP251K1P);

        let y1sqz1a = y1z1.multiply(&y1);
        let y1sqz1 = y1sqz1a.modulo(&SECP251K1P);

        let x1sq = x1.square();
        let x1sqz1a = x1sq.multiply(&THREE);
        let w = x1sqz1a.modulo(&SECP251K1P);

        let x3else = &x1.shift_left(3).multiply(&y1sqz1);
        let wsq = w.square();
        let x3c = wsq.subtract(x3else);
        let x3d = x3c.shift_left(1);
        let x3e = x3d.multiply(&y1z1);
        let x3 = x3e.modulo(&SECP251K1P);

        let y3a = w.multiply(&THREE);
        let y3b = y3a.multiply(&x1);
        let y3c = y3b.subtract(&y1sqz1.shift_left(1));
        let y3d = y3c.shift_left(2);
        let y3e = y3d.multiply(&y1sqz1);
        let y3f = y3e.subtract(&w.pow(3));
        let y3 = y3f.modulo(&SECP251K1P);
        let z3 = y1z1.pow(3).shift_left(3).modulo(&SECP251K1P);

        return Point {
            x: x3,
            y: y3,
            zinv: None,
            z: z3,
        };
    }

    pub fn twice_ip(&mut self) -> &mut Point {
        point_twice_ip(self);
        self
    }
}

#[profile()]
pub fn point_multiply(cls: &Point, k: &BigInt) -> Point {
    if 0 == k.signum() {
        // return Secp256k1.infinity;
    }
    let mut r = cls.clone();
    let e = k.clone();
    let h = e.multiply(&BigInt::nvb(3));

    let neg = cls.negate();
    for i in (1..=h.bit_length() - 2).rev() {
        let h_bit = h.test_bit(i);
        let e_bit = e.test_bit(i);
        r.twice_ip();

        if h_bit != e_bit {
            if h_bit {
                r.add_ip(&cls);
            } else {
                r.add_ip(&neg);
            }
        };
    }
    r
}

#[profile()]
pub fn point_add_ip(cls: &mut Point, other: &Point) -> () {
    let x1 = &cls.x;
    let y1 = &cls.y;

    let mut u = (other.y).clone();
    u.multiply_ip(&cls.z);
    u.subtract_ip(&y1.multiply(&other.z));
    u.modulo_ip(&SECP256K1.p);

    let mut v = (other.x).clone();
    v.multiply_ip(&cls.z);
    v.subtract_ip(&x1.multiply(&other.z));
    v.modulo_ip(&SECP256K1.p);

    let v_squared = v.square();
    let mut v3 = v_squared.multiply(&v);

    let x1v2 = x1.multiply(&v_squared);
    let mut zu2 = u.square().multiply(&cls.z);

    let mut x3 = zu2.clone();
    x3.subtract_ip(&x1v2.shift_left(1));
    x3.multiply_ip(&other.z);
    x3.subtract_ip(&v3);
    x3.multiply_ip(&v);
    x3.modulo_ip(&SECP256K1.p);
    cls.x = x3;

    cls.y.multiply_ip(&v3);
    zu2.multiply_ip(&u);

    let mut y3 = x1v2;
    y3.multiply_ip(&THREE);
    y3.multiply_ip(&u);
    y3.subtract_ip(&cls.y);
    y3.subtract_ip(&zu2);
    y3.multiply_ip(&other.z);
    y3.add_ip(&u.multiply(&v3));
    y3.modulo_ip(&SECP256K1.p);
    cls.y = y3;

    v3.multiply_ip(&cls.z);
    v3.multiply_ip(&other.z);
    v3.modulo(&SECP256K1.p);
    cls.z = v3;
}

#[profile()]
pub fn point_twice_ip(cls: &mut Point) {
    // y1z1 = y1 * cls.z;
    // y1z1 = y1z1 % SECP251K1P;

    // y1   = y1 * ((y1 * cls.z) % SECP251K1P);
    // y1   = y1 % SECP251K1P;

    let mut y1 = cls.y.clone();
    let mut y1z1 = y1.multiply(&cls.z);
    y1z1.modulo_ip(&SECP251K1P);

    y1.multiply_ip(&y1z1);
    y1.modulo_ip(&SECP251K1P);

    let x1 = &cls.x;
    let mut w = x1.square();
    w.multiply_ip(&THREE);
    w.modulo_ip(&SECP251K1P);

    let x3else = &x1.shift_left(3).multiply(&y1);

    let mut x3 = w.square();
    x3.subtract_ip(x3else);
    x3.l_shift_ip(1);
    x3.multiply_ip(&y1z1);
    x3.modulo_ip(&SECP251K1P);

    let mut y3 = w.multiply(&THREE);
    y3.multiply_ip(&x1);
    y3.subtract_ip(&y1.shift_left(1));
    y3.l_shift_ip(2);
    y3.multiply_ip(&y1);
    y3.subtract_ip(&w.pow(3));
    y3.modulo_ip(&SECP251K1P);
    cls.y = y3;
    cls.x = x3;

    y1z1 = y1z1.pow(3);
    y1z1.l_shift_ip(3);
    y1z1.modulo_ip(&SECP251K1P);
    cls.z = y1z1;
}
