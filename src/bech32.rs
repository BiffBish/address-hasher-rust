const ALPHABET: &str = "qpzry9x8gf2tvdw0s3jn54khce6mua7l";

fn polymod_step(pre: i32) -> i32 {
    let b = pre >> 25;
    return ((33554431 & pre) << 5)
        ^ (996825010 & -((b >> 0) & 1))
        ^ (642813549 & -((b >> 1) & 1))
        ^ (513874426 & -((b >> 2) & 1))
        ^ (1027748829 & -((b >> 3) & 1))
        ^ (705979059 & -((b >> 4) & 1));
}

fn convert(data: &[u8], in_bits: u32, out_bits: u32) -> Vec<u8> {
    let mut value = 0;
    let mut bits = 0;
    let max_v = (1 << out_bits) - 1;
    let mut result: Vec<u8> = Vec::new();
    for i in 0..data.len() {
        let va = i32::wrapping_shl(value, in_bits);
        value = (va | data[i] as i32) as i32;
        bits += in_bits;
        while bits >= out_bits {
            bits -= out_bits;

            let vb = i32::wrapping_shr(value as i32, bits) as u8;
            let v = vb & max_v as u8;
            result.push(v);
        }
    }
    if bits > 0 {
        result.push((i32::wrapping_shl(value, out_bits - bits) as u8) & max_v);
    }
    return result;
}

pub fn encode(prefix: &str, data: &[u8]) -> String {
    let mut chk = 1060121407;
    let mut result = prefix.to_string() + "1";
    for i in 0..data.len() {
        let x = data[i];
        chk = polymod_step(chk) ^ x as i32;
        result = result + &ALPHABET.chars().nth(x as usize).unwrap().to_string();
    }
    for _ in 0..6 {
        chk = polymod_step(chk);
    }

    chk ^= 1;

    for i in 0..6 {
        let v = (chk >> (5 * (5 - i))) & 31;
        result = result + &ALPHABET.chars().nth(v as usize).unwrap().to_string();
    }

    return result;
}

pub fn to_words(data: &[u8]) -> Vec<u8> {
    let res = convert(data, 8, 5);
    return res;
}
