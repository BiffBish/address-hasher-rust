pub fn main() -> [u8; 64] {
    let mut data = [0u8; 64];
    let _ah = 1779033703;
    let _bh = 3144134277;
    let _ch = 1013904242;
    let _dh = 2773480762;
    let _eh = 1359893119;
    let _fh = 2600822924;
    let _gh = 528734635;
    let _hh = 1541459225;
    let _al = 4089235720;
    let _bl = 2227873595;
    let _cl = 4271175723;
    let _dl = 1595750129;
    let _el = 2917565137;
    let _fl = 725511199;
    let _gl = 4215389547;
    let _hl = 327033209;
    fn write_u32_be(data: &mut [u8], offset: usize, value: u32) {
        data[offset] = (value >> 24) as u8;
        data[offset + 1] = (value >> 16) as u8;
        data[offset + 2] = (value >> 8) as u8;
        data[offset + 3] = value as u8;
    }
    write_u32_be(&mut data, 0, _ah);
    write_u32_be(&mut data, 4, _al);
    write_u32_be(&mut data, 8, _bh);
    write_u32_be(&mut data, 12, _bl);
    write_u32_be(&mut data, 16, _ch);
    write_u32_be(&mut data, 20, _cl);
    write_u32_be(&mut data, 24, _dh);
    write_u32_be(&mut data, 28, _dl);
    write_u32_be(&mut data, 32, _eh);
    write_u32_be(&mut data, 36, _el);
    write_u32_be(&mut data, 40, _fh);
    write_u32_be(&mut data, 44, _fl);
    write_u32_be(&mut data, 48, _gh);
    write_u32_be(&mut data, 52, _gl);
    write_u32_be(&mut data, 56, _hh);
    write_u32_be(&mut data, 60, _hl);
    return data;
}
