use crate::{big_int::BigInt, point::Point};

#[derive(Debug, Clone)]
pub struct Curve {
    pub p: BigInt,
    pub a: BigInt,
    pub n: BigInt,
    pub g: Point,
    pub infinity: Point,
    pub p_length: usize,
}

pub static SECP251K1P: once_cell::sync::Lazy<BigInt> = once_cell::sync::Lazy::new(|| {
    BigInt::new_from_iterator(
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe,
            0xff, 0xff, 0xfc, 0x2f,
        ]
        .iter(),
    )
});

impl Curve {
    pub fn new() -> Curve {
        let x = BigInt::new_from_iterator(
            [
                0x79, 0xbe, 0x66, 0x7e, 0xf9, 0xdc, 0xbb, 0xac, 0x55, 0xa0, 0x62, 0x95, 0xce, 0x87,
                0x0b, 0x07, 0x02, 0x9b, 0xfc, 0xdb, 0x2d, 0xce, 0x28, 0xd9, 0x59, 0xf2, 0x81, 0x5b,
                0x16, 0xf8, 0x17, 0x98,
            ]
            .iter(),
        );

        let y = BigInt::new_from_iterator(
            [
                0x48, 0x3a, 0xda, 0x77, 0x26, 0xa3, 0xc4, 0x65, 0x5d, 0xa4, 0xfb, 0xfc, 0x0e, 0x11,
                0x08, 0xa8, 0xfd, 0x17, 0xb4, 0x48, 0xa6, 0x85, 0x54, 0x19, 0x9c, 0x47, 0xd0, 0x8f,
                0xfb, 0x10, 0xd4, 0xb8,
            ]
            .iter(),
        );

        let c = Curve {
            p: SECP251K1P.clone(),
            a: BigInt::new_from_iterator([0x00].iter()),
            g: Point::from_affine(x, y),
            n: BigInt::new_from_iterator(
                [
                    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                    0xff, 0xff, 0xfe, 0xba, 0xae, 0xdc, 0xe6, 0xaf, 0x48, 0xa0, 0x3b, 0xbf, 0xd2,
                    0x5e, 0x8c, 0xd0, 0x36, 0x41, 0x41,
                ]
                .iter(),
            ),
            infinity: Point::new(BigInt::nvb(0), BigInt::nvb(0), BigInt::nvb(0)),
            p_length: 32,
        };

        c
    }
}
