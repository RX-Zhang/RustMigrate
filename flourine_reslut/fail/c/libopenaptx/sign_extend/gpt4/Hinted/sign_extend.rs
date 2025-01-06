
fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = 8 * std::mem::size_of::<i32>() as u32 - bits;
    ((val as u32) << shift) as i32 >> shift
}

fn main() {
    println!("{}", sign_extend(12800, 1));
    println!("{}", sign_extend(-385351616, 8));
    println!("{}", sign_extend(232, 7995392));
    println!("{}", sign_extend(489, 65556));
}
