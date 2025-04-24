pub fn read_u8(buf: &[u8], offset: usize) -> u8 {
    buf.get(offset).copied().unwrap_or(0)
}
pub fn read_u16(buf: &[u8], offset: usize) -> u16 {
    let lo = read_u8(buf, offset) as u16;
    let hi = read_u8(buf, offset + 1) as u16;
    hi << 8 | lo
}
pub fn read_u32(buf: &[u8], offset: usize) -> u32 {
    let b0 = read_u8(buf, offset) as u32;
    let b1 = read_u8(buf, offset + 1) as u32;
    let b2 = read_u8(buf, offset + 2) as u32;
    let b3 = read_u8(buf, offset + 3) as u32;
    (b3 << 24) | (b2 << 16) | (b1 << 8) | b0
}
pub fn gba_offset(abs: usize) -> usize {
    abs - 0x0200_0000
}
