use crate::{
    bigInt::{BigInt, THREE},
    curve::{Curve, SECP251K1P},
    Secp256k1,
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
        let z = crate::bigInt::ONE.clone();

        let point = Point {
            x: x,
            y: y,
            zinv: None,
            z: z,
        };
        point
    }
    pub fn multiply(&self, k: &BigInt) -> Point {
        if 0 == k.signum() {
            // return Secp256k1.infinity;
        }
        let mut r = self.clone();
        let e = k.clone();
        let h = e.multiply(&BigInt::nvb(3));

        let neg = self.negate();
        for i in (1..=h.bit_length() - 2).rev() {
            let hBit = h.test_bit(i);
            let eBit = e.test_bit(i);
            r = r.twice();

            if hBit != eBit {
                if hBit {
                    r = r.add(&self);
                } else {
                    r = r.add(&neg);
                }
            };
        }
        r
    }

    pub fn affineX(&self) -> BigInt {
        let binding = self.z.mod_inverse(&Secp256k1.p);
        let zinv = match &self.zinv {
            Some(zinv) => zinv,
            None => &binding,
        };

        // return this.x.multiply(this.zInv).mod(this.curve.p);
        self.x.multiply(zinv).modulo(&Secp256k1.p)
    }

    pub fn affineY(&self) -> BigInt {
        let binding = self.z.mod_inverse(&Secp256k1.p);
        let zinv = match &self.zinv {
            Some(zinv) => zinv,
            None => &binding,
        };
        // return this.y.multiply(this.zInv).mod(this.curve.p);
        self.y.multiply(zinv).modulo(&Secp256k1.p)
    }

    // compressed is true by default
    pub fn getEncoded(&self, compressed: bool) -> Vec<u8> {
        let compressed = true;
        // if (
        //     Secp256k1.isInfinity(self)
        // )
        //     {
        //     return Buffer.alloc(1, 0);
        // }
        let x = self.affineX();
        let y = self.affineY();

        let mut buffer = vec![0; 33];
        buffer[0] = if y.isEven() { 2 } else { 3 };
        let byteArray = x.to_byte_array_unsigned();
        buffer[1..].copy_from_slice(&byteArray[byteArray.len() - 32..]);
        return buffer;
    }
    // pub fn add_ip(&mut self, b: &Point) -> Point {
    //     // if (Secp256k1.isInfinity(self)) {return b;}
    //     // if (Secp256k1.isInfinity(b)) {return self;}

    //     // b.subtract();
    //     // x1.subtract(a)

    //     let x1 = self.x;
    //     let  y1 = self.y;
    //     let  x2 = b.x;

    //     let  u = b.y.multiply(&self.z).subtract(&y1.multiply(&b.z)).mod(Secp256k1.p);
    //     // let  v = x2.multiply(self.z).subtract(x1.multiply(b.z)).mod(Secp256k1.p);
    //     // if (0 === v.signum()) {
    //     //     return 0 === u.signum() ? self.twice() : Secp256k1.infinity;
    //     // }
    //     // let v2 = v.square();
    //     // let v3 = v2.multiply(v);
    //     // let x1v2 = x1.multiply(v2);
    //     // let zu2 = u.square().multiply(self.z);
    //     // let x3 = zu2
    //     //     .subtract(x1v2.shiftLeft(1))
    //     //     .multiply(b.z)
    //     //     .subtract(v3)
    //     //     .multiply(v)
    //     //     .mod(self.curve.p),
    //     // y3 = x1v2
    //     //     .multiply(THREE)
    //     //     .multiply(u)
    //     //     .subtract(y1.multiply(v3))
    //     //     .subtract(zu2.multiply(u))
    //     //     .multiply(b.z)
    //     //     .add(u.multiply(v3))
    //     //     .mod(self.curve.p),
    //     // z3 = v3.multiply(self.z).multiply(b.z).mod(self.curve.p);
    //     // return new Point(self.curve, x3, y3, z3);
    // }

    pub fn add(&self, a: &Point) -> Point {
        // if (this.curve.isInfinity(this)) return b;
        // if (this.curve.isInfinity(b)) return this;
        // var x1 = this.x,
        // y1 = this.y,
        // x2 = b.x,
        // u = b.y.multiply(this.z).subtract(y1.multiply(b.z)).mod(this.curve.p),
        // v = x2.multiply(this.z).subtract(x1.multiply(b.z)).mod(this.curve.p);
        // if (0 === v.signum())
        // return 0 === u.signum() ? this.twice() : this.curve.infinity;
        // var v2 = v.square(),
        // v3 = v2.multiply(v),
        // x1v2 = x1.multiply(v2),
        // zu2 = u.square().multiply(this.z),
        // x3 = zu2
        //     .subtract(x1v2.shiftLeft(1))
        //     .multiply(b.z)
        //     .subtract(v3)
        //     .multiply(v)
        //     .mod(this.curve.p),
        // y3 = x1v2
        //     .multiply(THREE)
        //     .multiply(u)
        //     .subtract(y1.multiply(v3))
        //     .subtract(zu2.multiply(u))
        //     .multiply(b.z)
        //     .add(u.multiply(v3))
        //     .mod(this.curve.p),
        // z3 = v3.multiply(this.z).multiply(b.z).mod(this.curve.p);
        // return new Point(this.curve, x3, y3, z3);

        // if self.isInfinity() {
        //     return a.clone();
        // }
        // if a.isInfinity() {
        //     return self.clone();
        // }
        let x1 = &self.x;
        let y1 = &self.y;
        let x2 = &a.x;
        let u =
            a.y.multiply(&self.z)
                .subtract(&y1.multiply(&a.z))
                .modulo(&Secp256k1.p);
        let v = x2
            .multiply(&self.z)
            .subtract(&x1.multiply(&a.z))
            .modulo(&Secp256k1.p);
        // if v.is_zero() {
        //     if u.is_zero() {
        //         return self.twice();
        //     }
        //     return Secp256k1.infinity.clone();
        // }
        let v2 = v.square();
        let v3 = v2.multiply(&v);
        let x1v2 = x1.multiply(&v2);
        let zu2 = u.square().multiply(&self.z);
        let x3 = zu2
            .subtract(&x1v2.shift_left(1))
            .multiply(&a.z)
            .subtract(&v3)
            .multiply(&v)
            .modulo(&Secp256k1.p);
        let y3 = x1v2
            .multiply(&THREE)
            .multiply(&u)
            .subtract(&y1.multiply(&v3))
            .subtract(&zu2.multiply(&u))
            .multiply(&a.z)
            .add(&u.multiply(&v3))
            .modulo(&Secp256k1.p);
        let z3 = v3.multiply(&self.z).multiply(&a.z).modulo(&Secp256k1.p);
        return Point {
            x: x3,
            y: y3,
            zinv: None,
            z: z3,
        };
    }
    pub fn negate(&self) -> Point {
        // var y = this.curve.p.subtract(this.y);
        // return new Point(this.curve, this.x, y, this.z);
        let y = Secp256k1.p.subtract(&self.y);
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
        //  ^^ This matches
        let w = x1sqz1a.modulo(&SECP251K1P);

        let x3else = &x1.shift_left(3).multiply(&y1sqz1);
        let wsq = w.square();
        let x3c = wsq.subtract(x3else);
        let x3d = x3c.shift_left(1);
        let x3e = x3d.multiply(&y1z1);
        let x3 = x3e.modulo(&SECP251K1P);

        // let y3 = w
        let y3a = w.multiply(&THREE);
        let y3b = y3a.multiply(&x1);
        let y3c = y3b.subtract(&y1sqz1.shift_left(1));
        let y3d = y3c.shift_left(2);
        let y3e = y3d.multiply(&y1sqz1);
        let y3f = y3e.subtract(&w.pow(3));
        let y3 = y3f.modulo(&SECP251K1P);
        let z3 = y1z1.pow(3).shift_left(3).modulo(&SECP251K1P);

        // panic!();
        return Point {
            x: x3,
            y: y3,
            zinv: None,
            z: z3,
        };
    }
}
