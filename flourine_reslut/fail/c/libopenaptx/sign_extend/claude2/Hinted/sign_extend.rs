
use std::boxed::Box;
use std::mem;

fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = (mem::size_of::<i32>() * 8) as u32 - bits;
    unsafe {
        let mut val = Box::new(val);
        let val_ptr = &mut *val as *mut i32 as *mut u32;
        *val_ptr = (*val_ptr).wrapping_shl(shift);
        let shifted = *val_ptr as i32;
        shifted.wrapping_shr(shift)
    }
}
