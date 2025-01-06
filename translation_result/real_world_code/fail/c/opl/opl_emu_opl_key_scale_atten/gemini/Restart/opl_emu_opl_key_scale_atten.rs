
const OPL_MAX: fn(i32, i32) -> i32 = |a, b| if a > b { a } else { b };

fn opl_emu_opl_key_scale_atten(block: i32, fnum_4msb: i32) -> i32 {
    static FNUM_TO_ATTEN: [i8; 16] = [
        0, 24, 32, 37, 40, 43, 45, 47, 48, 50, 51, 52, 53, 54, 55, 56,
    ];
    let result = FNUM_TO_ATTEN[fnum_4msb as usize] as i32 - 8 * (block ^ 7);
    OPL_MAX(0, result)
}
