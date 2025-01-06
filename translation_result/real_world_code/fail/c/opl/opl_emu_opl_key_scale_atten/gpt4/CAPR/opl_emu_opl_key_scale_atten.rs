
fn opl_max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn opl_emu_opl_key_scale_atten(block: u32, fnum_4msb: u32) -> u32 {
    // Using a Box pointer for the array as per the constraints
    let fnum_to_atten: Box<[i32; 16]> = Box::new([0, 24, 32, 37, 40, 43, 45, 47, 48, 50, 51, 52, 53, 54, 55, 56]);
    let fnum_index = (fnum_4msb & 0xF) as usize; // Ensure the index is within bounds
    // Convert block to i32 while preserving overflow behavior (simulates C unsigned to signed casting)
    let block_adjusted = (block.wrapping_sub(7)).wrapping_mul(8) as i32;
    
    let result = fnum_to_atten[fnum_index].wrapping_sub(block_adjusted);
    opl_max(0, result) as u32
}
