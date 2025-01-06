

use std::i32;

fn clip(a: i32, amin: i32, amax: i32) -> i32 {
let clipped = if a < amin {
amin
} else if a > amax {
amax
} else {
a
};
clipped
}

fn add_with_wrap(a: i32, b: i32) -> i32 {
a.wrapping_add(b)
}

