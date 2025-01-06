
use std::mem;

const QUANTIZATION_FACTORS: [i16; 32] = [
2048, 2304, 2560, 2816, 3072, 3328, 3584, 3840,
4096, 4352, 4608, 4864, 5120, 5376, 5632, 5888,
6144, 6400, 6656, 6912, 7168, 7424, 7680, 7936,
8192, 8448, 8704, 8960, 9216, 9472, 9728, 9984,
];

fn main() {
let mut a: i16 = 9000;
let b: i16 = 7000;

let c: i16 = a.wrapping_add(b);

println!("Sum is: {}", c);

let boxed_d: Box<i16> = Box::new(c);

println!("Value of boxed_d: {}", boxed_d);

let e: i16 = *boxed_d;

println!("Value of e: {}", e);

let mut f: i16 = 10000;

let g: i16 = f.wrapping_add(mem::take(&mut f));

println!("Value of g: {}", g);
}
