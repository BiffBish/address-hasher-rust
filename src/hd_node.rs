use crate::{
    profile, Profile, IS_PROFILE_RECONCILING, IS_PROFILING, PROFILING_DEPTH, PROFILING_MAP,
    PROFILING_PATH,
};
use colored::Colorize;
use std::hint::black_box;

use crate::big_int::BigInt;
use crate::ec_pair::ECPair;
use crate::hmac::Hmac;
use crate::rmd160hash::Rrd160Hash;
use crate::sha256hash::Sha256Hash;
use crate::SECP256K1;
#[derive(Debug, Clone)]
pub struct HDNode {
    pub key_pair: ECPair,
    pub chain_code: [u8; 32],
    pub depth: u8,
    pub index: u8,
    pub parent_fingerprint: u32,
}
impl HDNode {
    pub fn new(key_pair: ECPair, chain_code: [u8; 32]) -> HDNode {
        hd_node_new(key_pair, chain_code)
    }

    pub fn from_seed_buffer(seed: &Vec<u8>) -> HDNode {
        #[profile(no_sub)]
        fn from_seed_buffer(seed: &Vec<u8>) -> HDNode {
            let master_secret = b"Bitcoin seed";
            let data = Hmac::new(master_secret.to_vec())._encrypt(seed);

            let mut big_int = BigInt::new();
            let p_il = big_int.from_iterator(data[0..32].iter());

            let mut p_ir = [0; 32];
            for i in 0..32 {
                p_ir[i] = data[i + 32];
            }

            HDNode::new(ECPair::new(Some(p_il.clone()), None, true), p_ir)
        }

        from_seed_buffer(seed)
    }

    #[profile()]
    pub fn derive(&mut self, index: u8) -> HDNode {
        let derived_key_pair;
        let mut data = self.key_pair.get_public_key_buffer();
        data.push(0);
        data.push(0);
        data.push(0);
        data.push(0);

        let mut big_int = BigInt::new();
        let i = Hmac::new(self.chain_code.to_vec())._encrypt(&data);
        let ir = &i[32..64];
        let p_il = big_int.from_iterator(i[0..32].iter());

        if p_il.compare_to(&SECP256K1.n) >= 0 {
            return self.derive(index + 1);
        }
        if self.is_neutered() {
            let ki = SECP256K1.g.multiply(p_il).add(&self.key_pair.q);
            // if (Secp256k1.isInfinity(Ki)) {
            //     return self.derive(index + 1);
            // }
            derived_key_pair = ECPair::new(None, Some(ki), true)
        } else {
            let kpd = self.key_pair.d.as_ref().unwrap();

            let ki = p_il.add(&kpd).modulo(&SECP256K1.n);
            if 0 == ki.signum() {
                return self.derive(index + 1);
            }
            derived_key_pair = ECPair::new(Some(ki), None, true);
        }
        let mut irarray = [0; 32];
        for i in 0..32 {
            irarray[i] = ir[i];
        }
        let mut hd = HDNode::new(derived_key_pair, irarray);

        hd.depth = self.depth + 1;
        hd.index = index;
        let parent_fingerprint = self.get_fingerprint();
        hd.parent_fingerprint = parent_fingerprint[0] as u32
            | (parent_fingerprint[1] as u32) << 8
            | (parent_fingerprint[2] as u32) << 16
            | (parent_fingerprint[3] as u32) << 24;
        return hd;
    }

    #[profile()]
    pub fn get_identifier(&mut self) -> Vec<u8> {
        let pub_key_buffer = self.key_pair.get_public_key_buffer();

        let mut sha = Sha256Hash::new();
        sha.update(&pub_key_buffer);
        let digest = sha.digest();

        let mut rmd = Rrd160Hash::new();
        rmd.update(&digest);
        let out = rmd.digest();

        out
    }

    #[profile()]
    pub fn get_fingerprint(&mut self) -> Vec<u8> {
        let identifier = self.get_identifier();
        identifier[0..4].to_vec()
    }

    fn is_neutered(&self) -> bool {
        self.key_pair.d.is_none()
    }
}

#[profile()]
pub fn hd_node_new(key_pair: ECPair, chain_code: [u8; 32]) -> HDNode {
    HDNode {
        key_pair,
        chain_code,
        depth: 0,
        index: 0,
        parent_fingerprint: 0,
    }
}
