use crate::{bigInt::BigInt, curve, point::Point};
pub struct bip32 {
    pub public: u32,
    pub private: u32,
}
pub struct Network {
    pub messagePrefix: &'static str,
    pub bech32: &'static str,
    pub bip32: bip32,
    pub pubKeyHash: u8,
    pub scriptHash: u8,
    pub wif: u8,
}

pub static NETWORK: once_cell::sync::Lazy<Network> = once_cell::sync::Lazy::new(|| Network {
    messagePrefix: "\x18Bitcoin Signed Message:\n",
    bech32: "bc",
    bip32: bip32 {
        public: 0x0488B21E,
        private: 0x0488ADE4,
    },
    pubKeyHash: 0x17,
    scriptHash: 0x0a,
    wif: 0x80,
});

#[derive(Debug, Clone)]
pub struct ECPair {
    pub d: Option<BigInt>,
    pub Q: Point,
    pub compressed: bool,
    pub curve: curve::Curve,
}
impl ECPair {
    pub fn new(d: Option<BigInt>, Q: Option<Point>, compressed: bool) -> ECPair {
        let mut Q = Q;
        let mut d = d;
        if Q.is_none() && d.is_none() {
            panic!("Invalid arguments, expected Q or d");
        }
        let curve = curve::Curve::new();
        if Q.is_none() {
            let G = &curve.G;
            let dUn = d.unwrap();

            Q = Some(G.multiply(&dUn));
            d = Some(dUn);
        }

        ECPair {
            d: d,
            Q: Q.unwrap(),
            compressed: compressed,
            curve: curve,
        }
    }

    pub fn getPublicKeyBuffer(&self) -> Vec<u8> {
        return self.Q.getEncoded(true);
    }
}
