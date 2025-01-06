

fn main() {
use std::boxed::Box;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vector {
x: i32,
y: i32,
}

impl Vector {
fn add(self, other: Vector) -> Vector {
Vector {
x: self.x.wrapping_add(other.x),
y: self.y.wrapping_add(other.y),
}
}
}

fn add_vectors(v1: Vector, v2: Vector) -> Vector {
v1.add(v2)
}

let v1 = Vector { x: 1, y: 2 };
let v2 = Vector { x: 3, y: 4 };

let v3 = Box::new(v1).add(v2);

println!("v3 = {:?}", v3);
}
