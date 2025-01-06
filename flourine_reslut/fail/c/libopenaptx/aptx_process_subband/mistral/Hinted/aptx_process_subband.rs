

use std::boxed::Box;

fn main() {
let mut a: u8 = 250;
let mut b: u8 = 10;
let mut c: Box<u8> = Box::new(0);

*c = a.wrapping_add(b);
a = c.clone().wrapping_sub(b);
b = (*c).wrapping_sub(a);

println!("a = {}, b = {}, c = {}", a, b, *c);
}

