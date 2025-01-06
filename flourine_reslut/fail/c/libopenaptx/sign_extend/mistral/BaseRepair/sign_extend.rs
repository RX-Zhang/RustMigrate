

use std::boxed::Box;

fn main() {
    let mut x: Box<u32> = Box::new(0);
    let y: u32 = 5;
    *x = y;
    let z = *x;
    println!("value of z: {}", z);
}

