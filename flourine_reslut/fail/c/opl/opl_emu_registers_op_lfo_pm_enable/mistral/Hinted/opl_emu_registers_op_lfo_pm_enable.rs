

fn main() {
use std::boxed::Box;
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
x: i32,
y: i32,
}

impl Add for Point {
type Output = Point;

fn add(self, other: Point) -> Point {
Point {
x: self.x.wrapping_add(other.x),
y: self.y.wrapping_add(other.y),
}
}
}

let p1 = Box::new(Point { x: 1, y: 2 });
let p2 = Box::new(Point { x: 3, y: 4 });

let p3 = *p1 + *p2;

println!("{:?}", p3);
}
