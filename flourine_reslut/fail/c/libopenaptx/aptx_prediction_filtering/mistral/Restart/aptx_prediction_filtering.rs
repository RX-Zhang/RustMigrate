
use libc::int64_t;

fn main() {
println!("Hello, world!");
let a: int64_t = 9223372036854775807;
let b: int64_t = 1;
let c: int64_t = a.wrapping_add(b);
println!("c = {}", c);
}
