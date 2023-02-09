use crate::{big_int::BigInt, curve, point::Point};
use crate::{
    profile, Profile, IS_PROFILE_RECONCILING, IS_PROFILING, PROFILING_DEPTH, PROFILING_MAP,
    PROFILING_PATH,
};
use colored::Colorize;
use std::hint::black_box;

#[derive(Debug, Clone)]
pub struct ECPair {
    pub d: Option<BigInt>,
    pub q: Point,
    pub compressed: bool,
    pub curve: curve::Curve,
}
impl ECPair {
    pub fn new(d: Option<BigInt>, q: Option<Point>, compressed: bool) -> ECPair {
        let mut q = q;
        let mut d = d;
        if q.is_none() && d.is_none() {
            panic!("Invalid arguments, expected Q or d");
        }
        let curve = curve::Curve::new();
        if q.is_none() {
            let g = &curve.g;
            let d_un = d.unwrap();

            q = Some(g.multiply(&d_un));
            d = Some(d_un);
        }

        ECPair {
            d: d,
            q: q.unwrap(),
            compressed: compressed,
            curve: curve,
        }
    }
    #[profile(no_sub)]
    pub fn get_public_key_buffer(&self) -> Vec<u8> {
        return self.q.get_encoded();
    }
}
