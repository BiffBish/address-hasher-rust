use crate::bigInt::{self, BigInt};
use crate::ecPair::ECPair;
use crate::hmac::Hmac;
use crate::rmd160hash::Rrd160Hash;
use crate::sha256hash::Sha256Hash;
use crate::sha512hash::Sha512Hash;
use crate::Secp256k1;

#[derive(Debug, Clone)]
pub struct HDNode {
    pub keyPair: ECPair,
    pub chainCode: [u8; 32],
    pub depth: u8,
    pub index: u8,
    pub parentFingerprint: u32,
}

impl HDNode {
    // static fromSeedBuffer(seed: Buffer, network: any) {
    //     const masterSecret = "Bitcoin seed";
    //     const masterSecretBuffer = Buffer.from(masterSecret);

    //     const I = new HmacLow(masterSecretBuffer)._encrypt(seed) as Buffer;
    //     // const I = convertToBuffer(preformHmac(masterSecretBuffer, seed));
    //     // I[0] : 144
    //     // I[1] : 137
    //     // I[2] : 173
    //     // I[3] : 157
    //     // I[4] : 162

    //     const pIL = BigI.fromBuffer(I.slice(0, 32));
    //     const pIR = I.slice(32, 64);
    //     return new HDNode(new ECPair(pIL, null, { network: network }), pIR);
    //   }

    pub fn new(keyPair: ECPair, chainCode: [u8; 32]) -> HDNode {
        HDNode {
            keyPair: keyPair,
            chainCode: chainCode,
            depth: 0,
            index: 0,
            parentFingerprint: 0,
        }
    }

    pub fn from_seed_buffer(seed: &Vec<u8>, network: &str) -> HDNode {
        let master_secret = b"Bitcoin seed";
        let mut I = Hmac::new(master_secret.to_vec())._encrypt(seed);

        let mut bigInt = BigInt::new();
        let pIL = bigInt.from_iterator(I[0..32].iter(), 256);
        let pIR = &I[32..64];
        // return new HDNode(new ECPair(pIL, null, { network: network }), pIR);

        // Convert pIR to [u8; 32]
        let mut pIR = [0; 32];
        for i in 0..32 {
            pIR[i] = I[i + 32];
        }

        HDNode::new(ECPair::new(Some(pIL.clone()), None, true), pIR)
    }

    // fn new(keyPair: ECPair, chainCode: &[u8]) -> HDNode {
    //     HDNode {
    //         keyPair: keyPair,
    //         chainCode: chainCode,
    //         depth: 0,
    //         index: 0,
    //         parentFingerprint: 0,
    //     }
    // }
    pub fn derive(&mut self, index: u8) -> HDNode {
        // console.log("[HDNode.derive] index", index);
        // !! Change to 37 when SHA512 pads out data automatically
        // let data = Vec::with_capacity(37);
        // self.keyPair.getPublicKeyBuffer().copy(data, 0);
        // data.writeUInt32BE(index, 33);

        // data = [...self.keyPair.getPublicKeyBuffer(), 0,0,0,0];

        let derivedKeyPair;
        let mut data = self.keyPair.getPublicKeyBuffer();
        data.push(0);
        data.push(0);
        data.push(0);
        data.push(0);

        let mut bigInt = BigInt::new();
        let mut I = Hmac::new(self.chainCode.to_vec())._encrypt(&data);
        let IL = &I[0..32];
        let IR = &I[32..64];
        let pIL = bigInt.from_iterator(I[0..32].iter(), 256);

        if (pIL.compareTo(&Secp256k1.n) >= 0) {
            return self.derive(index + 1);
        }
        if (self.isNeutered()) {
            let Ki = Secp256k1.G.multiply(pIL).add(&self.keyPair.Q);
            // if (Secp256k1.isInfinity(Ki)) {
            //     return self.derive(index + 1);
            // }
            derivedKeyPair = ECPair::new(None, Some(Ki), true)
        } else {
            let kpd = self.keyPair.d.as_ref().unwrap();

            let ki = pIL.add(&kpd).modulo(&Secp256k1.n);
            if (0 == ki.signum()) {
                return self.derive(index + 1);
            }
            derivedKeyPair = ECPair::new(Some(ki), None, true);
        }
        let mut IRArray = [0; 32];
        for i in 0..32 {
            IRArray[i] = IR[i];
        }
        let mut hd = HDNode::new(derivedKeyPair, IRArray);

        hd.depth = self.depth + 1;
        hd.index = index;
        let parentFingerprint = self.getFingerprint();
        hd.parentFingerprint = parentFingerprint[0] as u32
            | (parentFingerprint[1] as u32) << 8
            | (parentFingerprint[2] as u32) << 16
            | (parentFingerprint[3] as u32) << 24;
        return hd;
    }

    pub fn getIdentifier(&mut self) -> Vec<u8> {
        let pubKeyBuffer = self.keyPair.getPublicKeyBuffer();

        let mut sha = Sha256Hash::new();
        sha.update(&pubKeyBuffer);
        let digest = sha.digest();

        let mut rmd = Rrd160Hash::new();
        rmd.update(&digest);
        let out = rmd.digest();

        out
    }
    pub fn getFingerprint(&mut self) -> Vec<u8> {
        let identifier = self.getIdentifier();
        identifier[0..4].to_vec()
    }

    fn isNeutered(&self) -> bool {
        self.keyPair.d.is_none()
    }

    fn neutered(&self) -> HDNode {
        let mut neutered = HDNode::new(
            ECPair::new(None, Some(self.keyPair.Q.clone()), true),
            self.chainCode,
        );

        neutered.depth = self.depth;
        neutered.index = self.index;
        neutered.parentFingerprint = self.parentFingerprint;

        neutered
    }
}

fn main() {}
