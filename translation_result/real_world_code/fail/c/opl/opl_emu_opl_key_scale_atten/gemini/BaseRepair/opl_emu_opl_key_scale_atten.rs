

fn opl_emu_opl_key_scale_atten(block: u32, fnum_4msb: u32) -> u32 {
    const FNUM_TO_ATTEN: [u8; 16] = [0, 24, 32, 37, 40, 43, 45, 47, 48, 50, 51, 52, 53, 54, 55, 56];

    let fnum_4msb = fnum_4msb % 16;
    let result = FNUM_TO_ATTEN[fnum_4msb as usize] as i32 - 8 * ((block ^ 7) as i32);
    result.max(0) as u32
}