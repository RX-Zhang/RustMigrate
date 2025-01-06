

use std::mem;
use std::u32;

fn main() {
let x = 0u32;
let y = 1u32;
let z = u32::wrapping_add(x, y);
println!("Sum: {}", z);
let a = Box::new(4u32);
let b = Box::new(5u32);
let c = u32::wrapping_add(*a, *b);
println!("Sum: {}", c);
let mut d = Box::new(6u32);
*d = *d + 1;
println!("Value: {}", d);
let e = Box::new(7u32);
let f = mem::replace(&mut d, e);
println!("Value: {}", d);
println!("Replaced: {}", f);
}

