

fn main() {
use std::boxed::Box;
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vector {
x: i32,
y: i32,
}

impl Add for Vector {
type Output = Vector;

fn add(self, other: Vector) -> Vector {
Vector {
x: self.x.wrapping_add(other.x),
y: self.y.wrapping_add(other.y),
}
}
}

let v1 = Box::new(Vector { x: 1, y: 2 });
let v2 = Box::new(Vector { x: 3, y: 4 });

let v3 = *v1 + *v2;

println!("{:?}", v3);
}
